use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn sleepy(time: u64) {
    sleep(Duration::from_millis(time));
}

fn main() {
    let (sender, receiver) = channel();

    let s1 = sender.clone();
    let s2 = sender.clone();

    thread::spawn(move || {
        // take by value
        sleepy(1000); // 1 sec.
        s1.send(9).unwrap();
    });

    thread::spawn(move || {
        sleepy(1000); // 1 sec.
        s2.send(9).unwrap();
    });

    println!("{:?}", receiver.try_recv()); // blocking
    println!("{:?}", receiver.try_recv());
    //    println!("{:?}", receiver.recv()); // function of waiting forever
    println!("All done!");
}
