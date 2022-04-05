// Any trait
// downcast_ref::<Book>
// downcast_ref::<Magazine>

use std::any::Any;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn sleepy(time: u64) {
    sleep(Duration::from_millis(time));
}

#[derive(Debug)]
struct Book {
    name: String,
}

fn book() -> Box<dyn Any + Send> {
    // turn to trait object
    let book = Book {
        name: "My Book".to_string(),
    };
    Box::new(book)
}

fn magazine() -> Box<dyn Any + Send> {
    let magazine = Magazine {
        name: "Nice Magazine".to_string(),
    };
    Box::new(magazine)
}

#[derive(Debug)]
struct Magazine {
    name: String,
}

fn main() {
    let (sender, receiver) = channel();

    let s1 = sender.clone();
    let s2 = sender.clone();

    thread::spawn(move || {
        for _ in 0..5 {
            sleepy(100);
            s1.send(book()).unwrap();
        }
    });

    thread::spawn(move || {
        for _ in 0..5 {
            sleepy(50);
            s2.send(magazine()).unwrap();
        }
    });

    println!("{:?}", receiver.recv_timeout(Duration::from_millis(500))); // blocking
    println!("{:?}", receiver.recv_timeout(Duration::from_millis(500))); // blocking
    println!("All done!");
}
