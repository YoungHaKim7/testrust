// Channels
// mpsc
// Multiple Producer Single Consumer
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (sender, receiver) = channel();
    sender.send(9).unwrap();
    let received = receiver.recv().unwrap();
    println!("{received}");

    thread::spawn(|| {
        sender.send(9).unwrap();
    });

    thread::spawn(|| {
        sender.send(9).unwrap();
    });

        println!("{}", receiver.recv().unwrap();
        println!("{}", receiver.recv().unwrap();

}
