use std::time;
use tokio;

async fn sleep(duration: u64) {
    // fn - blocking
    tokio::time::sleep(time::Duration::from_millis(duration)).await;
}

async fn give_data() -> u8 {
    // impl Future<Output = u8>
    sleep(1000).await; // 1sec
    7
}

async fn give_data_again() -> u8 {
    sleep(1000).await;
    7
}

#[tokio::main]
async fn main() {
    let now = time::Instant::now();

    let number_one = give_data().await; // Did not poll yet let number_two = give_data_again();
    let number_two = give_data_again().await;

    println!("{:?}", number_one);

    println!("{:?}", now.elapsed());
}

