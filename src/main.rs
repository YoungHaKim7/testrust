use std::mem::align_of;

struct MyStruct {
    bunch_of_stuff: u8,
    more_stuff: u64,
}

fn main() {
    println!("{}", align_of::<MyStruct>());
}
