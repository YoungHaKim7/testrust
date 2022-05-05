use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)] // JSON, YAML
struct City {
    title: String,
    location_type: String,
    woeid: u32,
    latt_long: String,
}
fn main() {
    let body = reqwest::blocking::get("https://www.metaweather.com/api/location/search/?query=san")
        .unwrap()
        .text()
        .unwrap();

    println!("{body:?}");
}
