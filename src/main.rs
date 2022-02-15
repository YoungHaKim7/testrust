// Sting &str 다음 시간에 할 내용은 String과 &str의 차이
// &str 어떤 Reference다. 데이타가 안에 있음.
// Reference를 땡겨와야 하는데 원본 자료가 drop이 되서 사라진 경우
// 코드가 아주 위험한 상황이다.!
// segmentation fault
// use after free
// 이것을 해결한게 Rust의 LifeTime이다. !! 대박!!!

struct Book {
    title: &str,
}

fn main() {
    let my_book = Book { title: "my_title" };
}
