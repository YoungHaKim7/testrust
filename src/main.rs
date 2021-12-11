// It's trivial to copy the bytes

// Ownership and copy types
fn prints_number(number: i32) {
    println!("{}", number);
}

fn print_string(input: String) {
    println!("{}", input);
}

// copy - copy types
// clone - non-copy types
fn main() {
    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);

    let my_country = "Austria".to_string();
    print_string(my_country.clone());
    print_string(my_country);
    print_string(my_country);
}
