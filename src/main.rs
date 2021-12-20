#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}

fn main() {
    let canada = Country {
        population: 35_000_000,
        capital: "Ottawa".to_string(),
        leader_name: "Justin Trudeau".to_string(),
    };

    println!("The country is : {:#?}", canada);
}
