// Channels
// mpsc
// Multiple Producer Single Consumer
use std::sync::mpsc::channel;

fn main() {
    let (sender, receiver) = channel();
    sender.send(9);
}
