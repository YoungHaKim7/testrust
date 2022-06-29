fn main() {
    // "RUST_LOG" "ERROR" "INFO", "DEBUG"
    for v in std::env::vars() {
        println!("{v:?}")
    }
}

