use futures::join;
use std::time;
use tokio;

async fn sleep(duration: u64) {
    // fn - blocking
    tokio::time::sleep(time::Duration::from_millis(duration)).await;
}

async fn listen_for_data() -> u8 {
    // impl Future<Output = u8>
    sleep(1000).await; // 1sec
    7
}

async fn listen_for_error() {
    sleep(1000).await;
    println!("Got an error")
}

#[tokio::main]
async fn main() {
    let now = time::Instant::now();

    tokio::select!(
    data = listen_for_data() => println!("Got some data: {data}"),
    error = listen_for_error() => error
    );

    println!("{:?}", now.elapsed());
}
