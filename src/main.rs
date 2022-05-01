use lazy_static::lazy_static;
use once_cell::sync::OnceCell;

static ERROR_LISTENER: OnceCell<ErrorListener> = OnceCell::new();

#[derive(Debug)]
struct ErrorListener {
    url: String,
}

impl ErrorListener {
    // ErrorListener::get_listener()
    fn check_for_error(&self) -> Result<(), ()> {
        println!("Checking for error....");
        Ok(())
    }
    fn get_listener() -> &'static ErrorListener {
        ERROR_LISTENER
            .get()
            .expect("Huh? Where's the ErrorListener?!?")
    }
}

fn do_stuff() {
    let listener = ErrorListener::get_listener();
    listener.check_for_error();
}

fn check_something_else() {
    let listener = ErrorListener::get_listener();
    listener.check_for_error();
}

fn main() {
    ERROR_LISTENER
        .set(ErrorListener {
            url: "www.thoenhoe.com".to_string(),
        })
        .expect("Couldn't set ErrorListener for some reason");

    do_stuff();
    check_something_else();
}
