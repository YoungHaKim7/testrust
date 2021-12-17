// Control Flow and match

fn main() {
    //  expression-based language(Rust is a language based on expression.)
    let my_number: u8 = 5;

    let second_number = match my_number {
        // switch
        0 => 23,
        1 => 65,
        _ => 0,
    };

    println!("The second number is: {}", second_number);
}
