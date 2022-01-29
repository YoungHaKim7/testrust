// closure = anomymous functions that capture their environment
// a|nonymous = no name
// enclose => 주변에 변수가 있으면 맘대로 쓸 수 있다.
// enclose 쓰는 방법은 || = pipes

fn main () {
    let my_closure = || println!("This is a closure");

    my_closure();

}