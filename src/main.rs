// Collection types:
// ARRAYS     [ ] all the same type

// 어레이 많이 쓰는 용도는 buffer
fn main() {
    let array = [
        "1월", "2월", "3월", "4월", "5월", "6월", "7월", "8월", "9월", "10월", "11월", "12월",
    ]; // indexing
    println!("{:?}", array.get(3)); // first
}
