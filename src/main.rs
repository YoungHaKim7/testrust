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
#[derive(Debug)]
struct MyStruct {
    number: i32,
}

fn main() {
    let my_numbers = [8u8, 9, 10, 11]; // [u8; 4] 4bytes
    let new_number = unsafe { transmute::<[u8; 4], MyStruct>(my_numbers) };

    println!("{new_number:?}");
}
