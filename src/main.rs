// pub 가 있으면 밖에서 쓸수 있게 만들어준다.
// mod 는 모듈을 만들어
// client::InternetClient  이렇게 쓸수 있게 만들어 준다.
// Debug again
use client::InternetClient;

mod client {
    pub struct InternetClient {
        pub client_id: u32, //other stuff
    }
}

struct Customer<'a> {
    money: u32,
    name: &'a str,
    client: &'a InternetClient,
}
use std::fmt;

impl fmt::Debug for Customer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Customer")
            .field("money", &self.money)
            .field("name", &self.name)
            .finish()
    }
}

fn main() {
    let client = client::InternetClient { client_id: 0 };

    let customer1 = Customer {
        money: 6876,
        name: "Billy",
        client: &client,
    };

    println!("{customer1:?}");
}
