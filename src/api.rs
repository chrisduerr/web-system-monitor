use std::process::Command;
use json::JsonValue;
use regex::Regex;

fn get_cpu_usage() -> JsonValue {
    let mut cpus_data = JsonValue::new_object();

    let cpu_out = Command::new("grep").args(&["cpu", "/proc/stat"]).output().unwrap();
    let cpu_out = String::from_utf8_lossy(&cpu_out.stdout).to_string();

    let cpu_reg = Regex::new("cpu.*? ([0-9]+) [0-9]+ ([0-9]+) ([0-9]+)").unwrap();
    for (i, caps) in cpu_reg.captures_iter(&cpu_out).enumerate() {
        if i == 0 {
            cpus_data["all"] = "".into();
        } else {
            cpus_data[format!("thread{}", (i - 1))] = "".into();
        }
    }

    cpus_data
}

fn get_ram_usage() -> JsonValue {
    let mut ram_data = JsonValue::new_object();

    let top_out = Command::new("top").arg("-bn1").output().unwrap();
    let top_out = String::from_utf8_lossy(&top_out.stdout).to_string();

    let ram_reg = Regex::new("Mem.*?([0-9]*\\.[0-9*])/([0-9]*\\.[0-9]*)").unwrap();
    for caps in ram_reg.captures_iter(&top_out) {
        ram_data["current"] = caps.at(1).unwrap().into();
        ram_data["max"] = caps.at(2).unwrap().into();
    }

    ram_data
}

fn get_cpu_temps(sensors_out: &String) -> JsonValue {
    let mut cpu_temp_data = JsonValue::new_object();

    let bundle_temp_reg = Regex::new("Physical.*?([+-][0-9]*\\.[0-9]*)°C").unwrap();
    let core_temp_reg = Regex::new("Core.*?([+-][0-9]*\\.[0-9]*)°C").unwrap();

    cpu_temp_data["all"] = bundle_temp_reg.captures(sensors_out).unwrap().at(1).unwrap().into();
    for (i, caps) in core_temp_reg.captures_iter(sensors_out).enumerate() {
        cpu_temp_data[format!("core{}", i)] = caps.at(1).unwrap().into();
    }

    cpu_temp_data
}

fn get_gpu_temps(sensors_out: &String) -> JsonValue {
    let mut gpu_temp_data = JsonValue::new_object();

    let gpu_temp_reg = Regex::new("temp.*?([+-][0-9]*\\.[0-9]*)°C.*hyst").unwrap();
    for (i, caps) in gpu_temp_reg.captures_iter(sensors_out).enumerate() {
        gpu_temp_data[format!("gpu{}", i)] = caps.at(1).unwrap().into();
    }

    gpu_temp_data
}

fn get_temps() -> JsonValue {
    let mut tmps_data = JsonValue::new_object();

    let sensors_out = Command::new("sensors").output().unwrap();
    let sensors_out = String::from_utf8_lossy(&sensors_out.stdout).to_string();

    tmps_data["cpu"] = get_cpu_temps(&sensors_out);
    tmps_data["gpu"] = get_gpu_temps(&sensors_out);

    tmps_data
}

fn get_network_usage() -> JsonValue {
    let mut net_data = JsonValue::new_object();

    let net_out = Command::new("cat").arg("/proc/net/dev").output().unwrap();
    let net_out = String::from_utf8_lossy(&net_out.stdout).to_string();

    let net_reg = Regex::new("[ew].*? ([0-9]+) +([0-9]+ +){7}([0-9]+)").unwrap();

    let net_caps = net_reg.captures(&net_out).unwrap();
    net_data["in"] = net_caps.at(1).unwrap().into();
    net_data["out"] = net_caps.at(3).unwrap().into();

    net_data
}

pub fn get_raw_data() -> String {
    let mut raw_data = JsonValue::new_object();

    raw_data["cpu"] = get_cpu_usage();
    raw_data["ram"] = get_ram_usage();
    raw_data["temps"] = get_temps();
    raw_data["network"] = get_network_usage();

    raw_data.dump()
}
