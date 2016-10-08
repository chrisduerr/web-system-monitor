function get_data_async(theUrl) {
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function() {
        if (xmlHttp.readyState == 4 && xmlHttp.status == 200)
            update(xmlHttp.responseText);
    }
    xmlHttp.open("GET", theUrl, true);
    xmlHttp.send(null);
}

function update(response) {
    console.log(response);
    document.getElementById("testdiv").innerText = response;
    sleep(500).then(() => get_data_async("/get_raw_data"));
}

function sleep(time) {
    return new Promise((resolve) => setTimeout(resolve, time));
}

get_data_async("/get_raw_data");
