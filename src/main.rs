fn main() {
    std::env::set_var("RUST_LOG", "DEBUG");
    for v in std::env::vars() {
        println!("{v:?}")
    }
}

