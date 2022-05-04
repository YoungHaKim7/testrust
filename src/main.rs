fn main() {
    let body = reqwest::blocking::get("https://www.metaweather.com/api/location/search/?query=san")
        .unwrap()
        .text()
        .unwrap();

    println!("{body:?}");
}
