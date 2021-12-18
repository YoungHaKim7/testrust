fn main() {
    let sky = "cloudy"; //&str
    let temperature = "warm";

    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's not very nice today"),
        ("clear", "warm") => println!("It's a nice day"),
        _ => println!("Not sure what the weather is. "),
    }
}
