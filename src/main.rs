// Box - owned data on the heap
//
struct SomeStruct {
    name: String,
    number: u8,
    data: Box<[u8; 10000]>,
}

fn take_thing<T>(thing: T) {}

fn main() {
    let my_struct = SomeStruct {
        name: "Hi there".to_string(),
        number: 0,
        data: Box::new([9; 10000]),
    };

    println!("The struct is {} bytes", std::mem::size_of_val(&my_struct));
}

