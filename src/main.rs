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
    let mut string_1 = String::from("I am String 1");
    let mut string_2 = String::from("I am String 2");

    swap(&mut string_1, &mut string_2);

    println!("string_1: {string_1}, string_2: {string_2}");
}
