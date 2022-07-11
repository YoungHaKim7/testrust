// rust 1.63 beta code ___
use std::sync::Mutex;
use std::thread;

fn main() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    thread::scope(|s| {
        s.spawn(|| {
            println!("Hello from the first scoped thread");
            // We can borrow `a` here.
            dbg!(&a);
        });

        s.spawn(|| {
            println!("Hello from the first scoped thread");
            // We can even butably borrow `x` here,
            // because no other threads are using it.
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    });
    // After the scope, we can modify and access our vatiables again:
    a.push(4);
    println!("print x : {}", a.len());
}

