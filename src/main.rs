fn add_is_great(country_name: &mut String) {
    country_name.push_str(" is great! ");
    println!("Now it says: {}", country_name);
}

fn main() {
    let my_country = "캐나다".to_string();
    add_is_great(&my_country); //by reference
    add_is_great(&mut my_country); //by mutable reference
}
