fn match_number(input: i32) {
    match input {
        number @ 4 => println!(
            "{} is an unlucky number in China: multiplied by 2 is {}",
            number,
            number * 2
        ),
        number @ 13 => println!("{} is a lucky number in Italy. In bocca al lupo", number),
        _ => println!("Looks like a normal number"),
    }
}

fn main() {
    match_number(50);
    match_number(13);
    match_number(4);
}
