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
    my_macro!(98, 8, 6786, 7);
}
