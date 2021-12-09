fn main() {
    // String
    // &str ref str "string slice"

    /*
    String = Sized types   데이타는 heap에 있다. String은 어떤 구조의 타입이다. 크기를 알수 있기 때문에 컴파일 에러 안남.
    str = dynamic types     str은 데이타의 크기를 알 수 없다. 그래서
    &str = dynamic types     Reference를 해 준다.
    */
    let my_name = "David"; // &str
    let my_name1 = "David".to_string(); // String
    let other_name = String::from("David2");
    // growable + shrinkable
    let mut my_other_name = "David3".to_string();
    my_other_name.push('!');
    println!("{}", my_other_name);
}
