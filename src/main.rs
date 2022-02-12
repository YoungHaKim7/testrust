use std::cmp::{max, min};

fn main() {
    let my_vec = vec![-878, 879879, -98798, 0, 76756];

    let biggest = my_vec // Smallest
        .into_iter()
        .fold(i32::MAX, |num1, num2| min(num1, num2));
    println!("Smallest is : {biggest}");
}
