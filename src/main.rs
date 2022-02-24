#[derive(Debug)]
struct City {
    name: String,
    population: u32, 
    history: String
}

#[derive(Debug)]
struct CityData {
    names: Vec<String>,
    histories: Vec<String>
}

fn main() {
    let calgary = City {
        name: "Calgary".to_string(),
        population: 1_336_000,
        history: "Calgary was founded in blah blah blah".to_string()
    };

    let canada_cities = CityData {
        names: vec![calgary.name],
        histories:vec![calgary.history]
    };
    println!("Calgary's history is : {}", calgary.history);
}

