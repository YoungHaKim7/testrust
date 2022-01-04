// Option<T>

fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4]) // i32
    }
}
// .expect

fn main() {
    let new_vec = vec![1, 2, 3];
    let index = take_fifth(new_vec); // Option<i32>

    index.expect("Needed at least five items - make sure Vec has at least five");
}
