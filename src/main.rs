use std::num::ParseIntError;

// anyhow - crate 일반적인 에러를 만드는 라이브러리

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
