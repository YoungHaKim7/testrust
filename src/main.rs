#[derive(Debug)]
struct Book {
    title: String,
    year: u16,
}

fn main() {
    let my_book = Book {
        title: "Some title".to_string(),
        year: 1919,
    };
    let book_2 = Book {
        title: "Book 2".to_string(),
        year: 2020,
    };
    let year = my_book.year;
    println!("My book year: {}", my_book.year);
}
