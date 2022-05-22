// non-blocking
// blocking thread blocking
use std::time;
use tokio;

fn sleep(duration: u64) {
    // fn - blocking
    std::thread::sleep(time::Duration::from_millis(duration));
}

async fn give_data() -> u8 {
    // impl Future<Output = u8>
    sleep(1000); // 1sec
    7
}

async fn give_data_again() -> u8 {
    sleep(1000);
    7
}

#[tokio::main]
async fn main() {
    let now = time::Instant::now();

    let number_one = give_data().await; // Did not poll yet let number_two = give_data_again();
    let number_two = give_data_again();

    println!("{:?}", number_one);

    println!("{:?}", now.elapsed());
}
