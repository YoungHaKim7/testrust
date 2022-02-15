// Sting &str 다음 시간에 할 내용은 String과 &str의 차이

struct Book {
    title: String,
}

fn main() {
    let my_book = Book {
        title: "my_title".to_string(),
    };
}
