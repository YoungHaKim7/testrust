// fold

fn main() {
    let my_vec = vec![-878, 879879, 98798, 0, 76756];

    let biggest = my_vec // biggest
        .into_iter()
        .fold(i32::MIN, |num1, num2| if num1 > num2 { num1 } else { num2 });
    println!("Biggest is : {biggest}");
}
