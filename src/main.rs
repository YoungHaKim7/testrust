// 3 Generics
//
use std::fmt::Display;


fn print<T: Display>(input: T) {
    println!("Hi, I'm a {input}");
}

fn main () {
    print(8);

}
