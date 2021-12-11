fn add_is_great(mut country_name: String) {
    country_name.push_str(" is great! ");
    println!("Now it says: {}", country_name);
}

fn main() {
    let my_country = "대한민국".to_string();
    add_is_great(my_country);
}
