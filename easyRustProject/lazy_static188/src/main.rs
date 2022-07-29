use lazy_static::lazy_static;
use reqwest::Client;

struct ErrorListener {
    url: String,
    client: Client,
}

lazy_static! {
    static ref ERORR_LISTENER: ErrorListener = ErrorListener {
        url: "lksdjflksjdflksdjf".to_string(),
        client: Client::default(),
    };
}

fn main() {}
