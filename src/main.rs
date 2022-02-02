//.chars() - iterator of char
//.count() - counts nubmer of items in iterator

fn main () {
    let big_string = "Hello there, I am a &str";

    big_string.chars().for_each(|c| println!("{c}"));

}