// multiple threadscfg!(
// !green threads --Go 가 쓰는 거 가짜 thread
// rust는 진짜 thread를 쓴다.
use std::thread;

fn main() {
    thread::spawn(|| {
        println!("I am printing something");
    }); // spawn 파생하다라는 뜻.
    for _ in 0..10000 {
        let x = 8;
    }
}



