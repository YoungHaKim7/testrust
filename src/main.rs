// non-blocking
// blocking thread blocking
use std::time;
use tokio;

async fn give_data() -> u8 {
    // impl Future<Output = u8>
    7
}

async fn give_data_again() -> u8 {
    7
}

#[tokio::main]
async fn main() {
    let now = time::Instant::now();

    println!("{:?}", now.elapsed());
}
