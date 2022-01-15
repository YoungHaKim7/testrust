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
    let my_book2 = Book {
        title: "Book 2".to_string(),
        year: 2020,
    };

    println!("Got books: \n{:?}\n{:?}", my_book, my_book2);
}
