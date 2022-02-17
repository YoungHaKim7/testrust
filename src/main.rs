// static str
// borrowed str
// 'static
struct Book<'a> {
    // Generics T,U
    name: &'a str,
    second_name: &'a str,
}

fn main() {
    let my_book_title = "my_book_title".to_string();

    //    let my_book = Book {
    //        name: &my_book_title,
    //    };
}
