// reqwest 리퀘스트는 기본적으로  async다.

use futures::join;
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

    let number_one_fut = give_data(); // impl Fure<Output = u8>
    let number_two_fut = give_data_again(); // impl Fure<Output = u8>

    let (number_one, number_two) = join!(number_one_fut, number_two_fut); //Polling

    println!("{:?}", now.elapsed());
}

