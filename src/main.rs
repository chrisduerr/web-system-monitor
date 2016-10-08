#[macro_use]
extern crate nickel;
extern crate regex;
extern crate json;

mod api;

use nickel::{Nickel, StaticFilesHandler, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    server.get("/get_raw_data", middleware!(api::get_raw_data()));
    server.utilize(StaticFilesHandler::new("static/"));

    server.listen("127.0.0.1:3000").unwrap();
}
