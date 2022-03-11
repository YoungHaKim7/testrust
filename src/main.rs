// impl trait
use std::fmt::Display;

// caller 가 정함.펑션을 만든사람이 정함
// 이론적으로는 무조건 잘 알아야할 Generic!!!!
fn generic_function<T: Display>(input: T) {
    println!("{input}");
}

// 컴퓨터가 알아서 정함 ㅎㅎ
// 실전에서 젤 많이 쓰는 코드
fn impl_function(input: impl Display) {
    println!("{input}")
}
fn main () {
//    generic_function::<u8>(8);
//    impl_function::<u8>(8); 
}
