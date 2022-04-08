use anyhow::Error;

fn try_to_make_numbers(int: &str, float: &str) -> Result<(), Error> {
    let my_integer = int.parse::<i32>()?; // 8 Err(ParseIntError)
    let my_float = float.parse::<f64>()?;
    Ok(())
}



fn main() {
    let first_try = try_to_make_numbers("8", "tnohenthojek");
    let second_try = try_to_make_numbers("tdothed", "8.7");
    println!("{first_try:?}");
    println!("{second_try:?}");
}
