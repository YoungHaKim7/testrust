// impl trait
use std::fmt::Display;

// caller 가 정함.펑션을 만든사람이 정함
fn generic_function<T: Display>(input: T) {
    println!("{input}");
}

// 컴퓨터가 알아서 정함 ㅎㅎ
fn impl_function(input: impl Display) {
    println!("{input}")
}
fn main () {
//    generic_function::<u8>(8);
    impl_function::<u8>(8); 
}
