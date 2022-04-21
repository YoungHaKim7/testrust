use std::mem::align_of;

struct MyStruct {
    bunch_of_stuff: u8, // 1 byte
    more_stuff: u64,
} // /9

// ********
// *///////

fn main() {
    println!("{}", align_of::<MyStruct>());
    println!("{}", std::mem::size_of::<MyStruct>());
}
