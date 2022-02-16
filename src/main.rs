// String 조금 느리고 &str조금더 빠르다. 이왕이면 &str위주로 쓰자!!

struct Book<'booklifetime> {
    // Generics T, U와 비슷하다.
    name: &'booklifetime str,
}

fn main() {
    let my_book = Book { name: "my book" };
}
