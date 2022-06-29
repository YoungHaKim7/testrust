fn main() {
    for v in std::env::vars() {
        println!("{v:?}")
    }
}

