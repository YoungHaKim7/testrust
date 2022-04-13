use fastrand::Rng;

fn main() {
    let rng = Rng::new();
    for _ in 0..5 {
        println!("{}", rng.alphabetic());
    }
}
