macro_rules! six_or_printl {
    (6) => {
        6
    };
    () => {
        println!("You didn't give me six");
    };
}

fn main() {
    let my_num = six_or_printl!(6);
    println!("{my_num}");
}
