use std::mem::{replace, swap, take, transmute};

// //swap
// pub fn swap<T>(x: &mut T, y: &mut T)

// // replace
// pub fn replace<T>(dest: &mut T, src: T) -> T

// // take
// pub fn take<T>(dest: &mut T) -> T
// where
//     T: Default,
//     // transmute DON'T USE THIS ㅋㅋㅋㅋ

fn main() {
    let mut my_string = "I am a String".to_string();
    let taking_thing = take(&mut my_string);

    println!("{taking_thing}, old string: {my_string}");
}
