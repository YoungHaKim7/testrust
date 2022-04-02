use std::time::Instant;

fn main() {
    let time_1 = Instant::now();
    for _ in 0..1000 {
        let _ = String::from("I am a String to keep you busy");
    }
    let time_2 = Instant::now();
    println!("{:?}", time_2 - time_1);
}
