// The question mark operator
// ? 써 주면 될수도 있고 안될수도 있고 라는 뜻이 됨
// 에러가 생길것 같다 생각이 들면 ? 써서 방지해준다. 이유는 편하기 때문!!
// 난중에 .await?  서버에서 다른 코드를 기다렸다가 같이 실행하는 코딩명령어
// ? 코드를 써 주겠되면 무조건 Result<i32, ParseIntError> 를 꼭 써줘야한다!!
use std::num::ParseIntError;

let parsed_number = input.parse::<u16>?.to_string().parse::<u32>()?.to_string().parse::<i32>()?; // Add a ? each time to check and pass it on

fn parse_str(input: &str) -> Result<(), ParseIntError> {
    let parsed_number = input.parse::<i32>()?; // return Error
    println!("It worked! {}", parsed_number);
    Ok(())
}

fn main() {
    for item in vec!["Senven", "8", "9.0", "nice", "6060"] {
        parse_str(item);
    }
}
