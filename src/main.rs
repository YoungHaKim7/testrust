// leak
// memery leak
// Rust prevents memery leaks
// no use after free

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
