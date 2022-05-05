use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)] // JSON, YAML
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

    let our_cities: Vec<City> = serde_json::from_str(&body).unwrap();

    println!("{our_cities:#?}");
}
