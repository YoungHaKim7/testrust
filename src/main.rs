use tokio;

fn give_8() -> u8 {
    // blocking function
    8
}

async fn async_give_8() -> u8 {
    // impl Future<Output = u8>
    8
}

#[tokio::main]
async fn main() {
    let my_number = give_8();
    let my_async_number = async_give_8().await;
    println!("{my_async_number}");
}
