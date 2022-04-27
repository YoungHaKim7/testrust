macro_rules! my_macro {
    () => {
        println!("Let's print this;")
    };
    ($input:expr) => {
        my_macro!()
    };
    ($($input:expr),*) => {
        my_macro!();
    };
}

fn main() {
    my_macro!(vec![8, 9, 10]);
}
