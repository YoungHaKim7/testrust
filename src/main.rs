use std::cmp::PartialOrd;
use std::fmt::Display;

fn compare_and_print<DisplayType, CompareType>(
    statement: DisplayType,
    num_1: CompareType,
    num_2: CompareType,
) where
    DisplayType: Display,
    CompareType: Display + PartialOrd,
{
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
