use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let num_1 = Arc::new(Mutex::new(0));
    let num_2 = 0;
    let num_3 = 0;

    let clone_1 = Arc::clone(&num_1);
    let clone_2 = Arc::clone(&num_1);

    // Rust's closures 는 기본적으로 레퍼런스로 쓰려고 하기 때문에
    // move 로 해 줘야 한다.
    let mut join_vec = vec![];

    join_vec.push(thread::spawn(move || {
        println!("I am in thread 1");
        *clone_1.lock().unwrap() += 1;
    }));

    join_vec.push(thread::spawn(move || {
        println!("I am in thread 2");
        *clone_2.lock().unwrap() += 1;
    }));

    for handle in join_vec {
        handle.join().unwrap();
    }

    println!("{num_1:?}");
}

