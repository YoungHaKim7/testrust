use std::cmp::PartialOrd;
use std::fmt::Display;

fn compare_and_print<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
    println!(
        "{}! Is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2
    );
}

fn main() {
    compare_and_print("Listen up!", 9, 8);
}
