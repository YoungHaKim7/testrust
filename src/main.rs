// Cow 공식 문서 볼때 보라색 표시는 trait

//
fn main() {
    let string_1 = String::from("Hello there"); // From Trait
    let string_2 = "Hello there".to_string(); // Display Trait
    let string_3: String = "Hello there".into(); // From
    let string_4 = "Hello there".to_owned(); // ToOwned Trait , str -> String이 아닌 다른 타입으로
    // 바꾸고 싶을때 씀 Owned Type? Clone할 때 씀
    // str -> String
}
