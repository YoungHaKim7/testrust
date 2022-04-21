use std::mem::align_of;

struct MyStruct {
    bunch_of_stuff: u16, // 4 byte
    more_stuff: u32,
} // /8

// ****
// *///

fn main() {
    println!("{}", align_of::<MyStruct>());
    println!("{}", std::mem::size_of::<MyStruct>());
}
