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
            sleepy(1);
            s1.send(book()).unwrap();
        }
    });

    thread::spawn(move || {
        for _ in 0..5 {
            sleepy(1);
            s2.send(magazine()).unwrap();
        }
    });

    while let Ok(any_type) = receiver.recv() {
        if let Some(book) = any_type.downcast_ref::<Book>() {
            println!("Got a book: {book:?}");
        } else if let Some(magazine) = any_type.downcast_ref::<Magazine>() {
            println!("Got a magazine: {magazine:?}")
        } else {
            panic!("Expected a magazine or a book, what's going on");
        }
    }
}
