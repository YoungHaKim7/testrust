#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}

fn main() {
    let populatin = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader_name = "Justin Trudeau".to_string();
    let my_country = Country {
        population: populatin,
        capital: capital,
        leader_name: leader_name,
    };
}
