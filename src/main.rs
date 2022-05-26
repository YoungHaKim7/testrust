use futures::join;
use std::time;
use tokio;

async fn sleep(duration: u64) {
    // fn - blocking
    tokio::time::sleep(time::Duration::from_millis(duration)).await;
}

async fn listen_for_data() -> u8 {
    // impl Future<Output = u8>
    sleep(100).await; // 1sec
    7
}

async fn listen_for_error() {
    sleep(100).await;
    println!("Got an error")
}

#[tokio::main]
async fn main() {
    for _ in 0..10 {
        tokio::select!( // race await against each other
        data = listen_for_data() => println!("Got some data: {data}"),
        error = listen_for_error() => error
        );
    }
}
