use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use std::sync::Mutex;

lazy_static! {
    static ref LAZY_STATIC_LOG_INFO: Mutex<String> = Mutex::new(String::new());
}

static ONCECELL_LOG_INFO: OnceCell<Mutex<String>> = OnceCell::new();

// Rust 1.63 Good! awesome
static COOLER_LOG_INFO: Mutex<String> = Mutex::new(String::new());

// Rust 1.63 nightly code

fn main() {
    *LAZY_STATIC_LOG_INFO.lock().unwrap() = "Important imformation".to_string();
    ONCECELL_LOG_INFO.set(Mutex::new(String::new())).unwrap();
    *ONCECELL_LOG_INFO.get().unwrap().lock().unwrap() = "Important information".to_string();

    println!(
        "One: {:?} \n Two: {:?}",
        LAZY_STATIC_LOG_INFO.lock().expect("Could not lock mutes"),
        ONCECELL_LOG_INFO
    );
}
