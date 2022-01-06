use std::num::ParseIntError;

// anyhow - crate

fn parse_number(number: &str) -> Result<i32, ParseIntError> {
    number.parse()
}

fn main() {
    let mut result_vec = vec![];
    result_vec.push(parse_number("8"));
    result_vec.push(parse_number("tnohunthoe"));
    result_vec.push(parse_number("8"));

    for number in result_vec {
        println!("{:?}", number);
    }
}
