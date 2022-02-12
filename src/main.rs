use std::cmp::max;

fn main() {
    let my_vec = vec![-878, 879879, 98798, 0, 76756];

    let biggest = my_vec // biggest
        .into_iter()
        .fold(i32::MIN, |num1, num2| max(num1, num2));
    println!("Biggest is : {biggest}");
}
