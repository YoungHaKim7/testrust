// closure = anomymous functions that capture their environment
// a|nonymous = no name
// enclose => 주변에 변수가 있으면 맘대로 쓸 수 있다.
// enclose 쓰는 방법은 || = pipes

fn main () {
    let my_number = 10;
    let my_closure = |x: i32| println!("{}", x + my_number);

    my_closure(9);

}