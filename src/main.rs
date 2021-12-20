// struct = and 이거 들어있고 들어 있고 할때!!
// Rust에는 struct가 3가지가 있다.
//      1. unit struct - 아무것도 없고 이름만 씀 -"byte는 0이다."
//      2. tuple struct
//      3. named struct - 가장 많이 쓰고 제일 편리함.!!
// enum = or 선택해서 쓸 때 씀.... struct와 약간 비슷
//unit struct
struct FileDirectory;

// tuple struct
#[derive(Debug)] //attribute
struct Colour(u8, u8, u8);

// named struct
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}

fn main() {
    let 
}
