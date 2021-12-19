fn match_number(input: i32) {
    match input {
        number => println!("It's the number {}", number), // number라는 아무런거나 맞는거 변수 넣는다.
        number @ 0..=10 => println!("It's between 0 and 10. It's the number {}", number),
        _ => println!("It's greater than ten"),
    }
}

fn main() {}
