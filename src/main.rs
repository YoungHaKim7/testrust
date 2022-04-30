use lazy_static::lazy_static;

include_str!();
include_bytes!();
lazy_static! {
    static ref ERROR_LISTENER: ErrorListener = ErrorListener {
        url: "www.nthdidjf.com".to_string(),
    };
}

#[derive(Debug)]
struct ErrorListener {
    url: String,
}

impl ErrorListener {
    fn check_for_error(&self) -> Result<(), ()> {
        println!("Checking for error....");
        Ok(())
    }
}

fn do_stuff() {
    ERROR_LISTENER.check_for_error();
}

fn check_something_else() {
    ERROR_LISTENER.check_for_error();
}

fn main() {
    let some_bytes = include_bytes!(); // 5MB
    do_stuff();
    check_something_else();
}

