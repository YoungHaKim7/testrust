use std::mem::{size_of, size_of_val};

struct MyStruct {
    bunch_of_stuff: Box<[u32; 1000]>,
}

fn main() {
    println!("{} {}", size_of::<MyStruct>(), size_of_val("I am a &str"));
}
