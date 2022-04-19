// retain
// String pop
// from_utf8
// from_utf8_lossy
// split_off

fn main() {
    let mut my_string = String::from("Hello there");
    my_string.chars().count(); // split_off 쓸 길이를 미리 알아보는 코드
    let second_string = my_string.split_off(5);
    println!("{my_string}, {second_string}");
}
