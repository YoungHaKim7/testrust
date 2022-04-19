// retain
// String pop
// from_utf8
// from_utf8_lossy
// split_off

fn main() {
    let mut my_string = String::from("Hello there");
    let second_string = my_string.split_off(5);
    println!("{my_string}, {second_string}");
}
