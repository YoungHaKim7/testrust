use std::thread;

fn main() {
    let num_1 = 0; // 'static만 가능하다 지금  rust1.62 version 에서는
    let num_2 = 0;
    let num_3 = 0;

    thread::spawn(|| {
        println!("I am in thread 1");
    });

    thread::spawn(|| {
        println!("I am in thread 2");
    });

    for _ in 0..110000 {
        let _x = 1;
    }
}

