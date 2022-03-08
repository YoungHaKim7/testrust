// 3 Generics
use std::fmt::Display;

// concrete (Compiler)
fn print<T: Display>(input: T) {
    println!("Hi, I'm a {input}");
}

// fn print_i32<input: i32>(input: T) {
//     println!("Hi, I'm a {input}");
// }
//
// fn print_string<input: String>(input: T) {
//     println!("Hi, I'm a {input}");
// }
//

//impl trait
// concrete (Compiler)
fn print_2(input: impl Display) {
    println!("Hi, I'm a {input}");
}

// dynamic (Runtime)
fn print_3(input: Box<dyn Display>) {
     println!("Hi, I'm {input}");
 }
//vtable

fn main () {

    print(8);
    print(String::from("I am String"));
    print_2(9);
    print_3(Box::new(5));
    print_3(Box::new("I am String_Box_dyn"));

}
