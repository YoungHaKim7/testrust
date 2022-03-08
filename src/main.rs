// 3 Generics
use std::fmt::Display;

// concrete (Compiler)
fn print<T: Display>(input: T) {
    println!("Hi, I'm a {input}");
}
//impl trait
// concrete (Compiler)
fn print_2(input: impl Display) {
    println!("Hi, I'm a {input}");
}

// dynamic (Runtime)
fn print_3(input: Box<dyn Display>) {
     println!("Hi, I'm {input}");
 }

fn main () {
    print(8);
    print_2(9);
    print_3(Box::new(5));

}
