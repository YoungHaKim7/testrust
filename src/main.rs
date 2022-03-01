// multiple threadscfg!(
// !green threads --Go 가 쓰는 거 가짜 thread
// rust는 진짜 thread를 쓴다.
use std::thread;

fn main() {
    thread::spawn(|| {}); // spawn 파생하다라는 뜻.
}



