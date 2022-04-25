macro_rules! might_print {
    ($input:expr) => {
        println!("{:?}", $input);
    };
}

fn main() {
    might_print!(9);
    might_print!("Hi there");
    let my_vec = vec![8, 9, 10];
    might_print!(my_vec);
}
