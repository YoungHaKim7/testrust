// 'static 만 특별한 Lifetime이다.
// 이 예문이 가능한건 'static 뿐이다.
struct Book<'static> {
    name: &'static str,
}

fn main() {
    let my_book = Book { name: "my book" };
}
