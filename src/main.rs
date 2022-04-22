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
    let mut money = 60_000_000;
    let mut old_money = replace(&mut money, 70_000_000);

    println!("{money}, {old_money}");
}
