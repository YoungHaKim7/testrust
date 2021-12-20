// struct = and 이거 들어있고 들어 있고 할때!!
// Rust에는 struct가 3가지가 있다.
//      1. unit struct - 아무것도 없고 이름만 씀 -"byte는 0이다."
//      2. tuple struct
//      3.  struct
// enum = or 선택해서 쓸 때 씀.... struct와 약간 비슷

// tuple struct
struct Colour(u8, u8, u8);

fn main() {
    let my_colour = Colour(20, 50, 100);
    println!("The second colour is {}", my_colour.1);
}
