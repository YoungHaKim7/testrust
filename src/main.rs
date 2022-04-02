use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let time_1 = Instant::now();
    sleep(Duration::from_secs(5));
    println!("{:?}", time_1.elapsed()); // elapsed = time that passed
    println!("{:?}", time_1.elapsed()); // elapsed = time that passed
    println!("{:?}", time_1.elapsed()); // elapsed = time that passed
}
