// get과 비슷한거 .into_inner()
use std::cell::Cell;
// cell은 copy타입이 아니면 쓸 때가 없다. ㅡㅡ
// small copy types
// not thread safe
// set
// get

fn main() {
    let my_cell = Cell::new(String::from("I am a String"));
    my_cell.set(String::from("I am a String??!?!?!?!?"));
    let my_string = my_cell.get(); // 여기서 에러남 copy타입이 아니라서
}
