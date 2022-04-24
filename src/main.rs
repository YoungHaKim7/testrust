macro_rules! six_or_printl {
    (6) => {
        6
    };
    () => {
        println!("You didn't give me six");
    };
}

fn main() {
    six_or_printl!();
}
