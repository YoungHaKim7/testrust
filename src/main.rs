use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    for _ in 0..5 {
        println!("{}", rng.gen_range('a'..'행')); // gen_range(1,10);
    }
}
