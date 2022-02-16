struct Book<'booklifetime> {
    // Generics T, U와 비슷하다.
    name: &'booklifetime str,
}
use std::fmt::Display;

fn print_thing<T: Display>(input: T) {}

fn main() {
    let my_book = Book { name: "my book" };
}
