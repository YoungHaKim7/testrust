use std::mem::size_of_val;

#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
    all_populations: [u32; 5500],
}

fn main() {
    let population = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader_name = "Justin Trudeau".to_string();
    let my_country = Country {
        population,
        capital,
        leader_name,
        all_populations: [500; 5500],
    };

    println!("Country is {} bytes in size", size_of_val(&my_country));
}
