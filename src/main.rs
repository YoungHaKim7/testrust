fn main() {
    for v in std::env::vars() {
        println!("{v:?}")
    }

    println!("{}", std::env::var("LANGUAGE").unwrap());
}

