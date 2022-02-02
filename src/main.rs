//.chars() - iterator of char
//.count() - counts nubmer of items in iterator

fn main () {
    let big_string = "Hello there, I am a &str";

    println!("big_string has {} characters", big_string.chars().count());

}