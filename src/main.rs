// Control Flow and match

fn main() {
    // match
    let my_number = 5;

    match my_number {
        // switch
        0 => println!("It's a zero"), // => fat arrow
        1 => println!("It's a one"),
        _ => println!("It's a different number"), //  _ "I don't care"  "anything else"
    }
}
