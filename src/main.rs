// String 은 u8타입으로 UTF-8 로 변환?
// A UFT-8-encoded, growable string.
// The String type is the most common string type that has ownership over the contents of the string. It has a close relationship with its borrowed counterpart, the primitive str.
// pub struct Sting {
//     vec: Vec<u8>,
// }

#[derive(Debug)]
struct NeedsAStatic {
    name: &'static str,
}

fn get_our_data() -> String {
    "Data".to_string()
}

fn main() {
    // Vec<T> Box<T> // owned data
    let our_data = get_our_data(); // String
    let boxed_data = Box::new(our_data); // Box<String>
    let leaked_data = Box::leak(boxed_data); // &'static str

    let our_stuct = NeedsAStatic { name: leaked_data };

    println!("{our_stuct:?}");
}
