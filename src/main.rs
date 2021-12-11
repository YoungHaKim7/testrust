// OWNERSHIP
// move semantics

fn print_country(country_name: String) -> String {
    println!("My country is {}", country_name);
    country_name
}

fn main() {
    let mut country = "대 한 민 국! ".to_string();
    country = print_country(country);
    country = print_country(country);
    country = print_country(country);
}
