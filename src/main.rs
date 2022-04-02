use std::time::Instant;

fn main() {
    let time_1 = Instant::now();

    println!("{:?}", time_1.elapsed()); // elapsed = time that passed
    println!("{:?}", time_1.elapsed()); // elapsed = time that passed
    println!("{:?}", time_1.elapsed()); // elapsed = time that passed
}
