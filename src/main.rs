// 'static 만 특별한 Lifetime이다.
// 이 예문이 가능한건 'static 뿐이다.
struct Book<'book> {
    name: &'book str,
}

use std::fmt::Display;
fn print_thing<T: Display>(input: T) {}

fn main() {
    let my_book = Book { name: "my book" };
}
