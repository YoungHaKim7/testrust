const ERROR_LISTENER: ErrorListener = ErrorListener {
    url: "www.nthdidjf.com".to_string(),
};

#[derive(Debug)]
struct ErrorListener {
    url: String,
}

impl ErrorListener {
    fn check_for_error(&self) -> Result<(), ()> {
        Ok(())
    }
}

fn do_stuff() {
    ERROR_LISTENER.check_for_error();
}

fn main() {}

