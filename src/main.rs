macro_rules! might_print {
    (input) => {
        println!("", input);
    };
}

fn main() {
    might_print!(9);
    might_print!("Hi there");
}
