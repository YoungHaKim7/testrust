use std::io;

fn main() -> Result<(), std::io::Error> {
    println!("Please type something, or x to escape");
    let mut input_string = String::new(); // PartialEq<&str>

    while input_string != "x" {
        input_string.clear();
        io::stdin().read_line(&mut input_string)?;
    }
    Ok(())
}
