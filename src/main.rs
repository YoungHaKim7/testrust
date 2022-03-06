// Box - owned data on the heap
// Box = Smart Point

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

    let my_box = Box::new(9);

    println!("{my_box:?}");
    println!("{}", *my_box);

}

