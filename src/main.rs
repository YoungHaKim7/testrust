// rust 1.63 beta code ___
use std::sync::Mutex;
use std::thread;

fn main() {
    let num_1 = Mutex::new(0);
    let mut num_2 = 0;
    let num_3 = 0;

    thread::scope(|s| {
        s.spawn(|| {
            *num_1.lock().unwrap() += 1;
            num_2 += 10;
            println!("num_3 is : {num_3}");
        });

        s.spawn(|| {
            *num_1.lock().unwrap() += 1;
            // num_2 += 10;
            println!("num_3 is : {num_3}");
        });
    });

    println!("{num_1:?} {num_2} {num_3}");
}

