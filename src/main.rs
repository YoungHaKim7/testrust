use std::mem::size_of_val;

struct Numbers {
    one: u8,
    two: u8,
    three: u8,
}

#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}

fn main() {
    let population = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader_name = "Justin Trudeau".to_string();
    let my_country = Country {
        population,
        capital,
        leader_name,
    };

    println!("Country is {} bytes in size", size_of_val(&my_country));

    let numbers = Numbers {
        one: 8,
        two: 19,
        three: 20,
    };

    println!("size is {}", size_of_val(&numbers));
}
