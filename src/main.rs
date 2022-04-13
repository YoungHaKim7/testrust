use rand;

fn main() {
    for _ in 0..5 {
        let random_u16 = rand::random::<u16>();
        let random_char = rand::random::<char>();
        println!("{random_u16}, {random_char}");
    }
}
