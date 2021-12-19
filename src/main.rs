fn match_number(input: i32) {
    match input {
        number @ 0..=10 => println!("It's between 0 and 10. It's the number {}", number),
        _ => println!("It's greater than ten"),
    }
}

fn main() {
    match_number(10);
    match_number(2);
    match_number(100);
}
