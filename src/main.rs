// OWNERSHIP
// move semantics

fn print_country(country_name: &String) {
    println!("My country is {}", country_name);
}

fn main() {
    let country = "대 한 민 국! ".to_string();
    print_country(&country);
    print_country(&country);
    print_country(&country);
}
