use reqwest::Client;

struct ErrorListener {
    url: String,
    client: Client,
}

static ERORR_LISTENER: ErrorListener = ErrorListener {
    url: "lksdjflksjdflksdjf".to_string(),
    client: Client::default(),
};

fn main() {}
