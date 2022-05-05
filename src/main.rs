use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)] // JSON, YAML
struct City {
    title: String,
    location_type: String,
    woeid: u32,
    latt_long: String,
}
// impl Future
async fn give_u8() -> u8 {
    // impl future<Output = u8> lazy    .await 필요함
    8
}
fn main() {
    let body = reqwest::get("https://www.metaweather.com/api/location/search/?query=san")
        .unwrap()
        .text()
        .unwrap();

    let our_cities: Vec<City> = serde_json::from_str(&body).unwrap();

    println!("{our_cities:#?}");
}
