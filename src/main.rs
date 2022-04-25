macro_rules! print_anything {
    ($input:tt) => {
        let output = stringify!($input);
        println!("{output}");
    };
}

fn main() {
    print_anything!(9);
    let my_string = String::from("I am a String");
    print_anything!(my_string);
}
