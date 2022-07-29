use lazy_static::lazy_static;
use reqwest::Client;

struct ErrorListener {
    url: String,
    client: Client,
}

impl ErrorListener {
    fn check_for_error(&self) -> Result<(), ()> {
        Ok(())
    }
}

fn do_stuff() {
    println!("Doing stuff. Is there an error?");
    ERROR_LISTENER.check_for_error().unwrap();
}

fn check_something_else() {
    println!("Checking stuff. Any errors?");
    ERROR_LISTENER.check_for_error().unwrap();
}

lazy_static! {
    static ref ERROR_LISTENER: ErrorListener = ErrorListener {
        url: "lksdjflksjdflksdjf".to_string(),
        client: Client::default(),
    };
}

fn main() {
    do_stuff();
    check_something_else();
}
