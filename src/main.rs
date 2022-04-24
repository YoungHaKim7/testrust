macro_rules! might_print {
    (Hey there 하하하 will this still work? ) => {
        println!("You guessed the secret message");
    };
    () => {
        println!("You didn't guess it")
    };
}

fn main() {
    might_print!(Hey there 하하하 will this still work? );
}
