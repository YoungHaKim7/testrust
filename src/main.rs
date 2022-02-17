// 'static 만 특별한 Lifetime이다.
// 이 예문이 가능한건 'static 뿐이다.
// static str
// borrowed str 이 예문은 borrowed str예문이다.!!!!
// 'static
struct Book {
    // Generics T,U
    name: &'static str,
}

fn main() {
    let my_book_title = "my_book_title".to_string();

    let my_book = Book {
        name: &my_book_title,
    };
}
