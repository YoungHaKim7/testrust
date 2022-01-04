// Option<T>

fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4]) // i32
    }
}
// wrap in an Option

fn main() {
    let new_vec = vec![1, 2, 3, 4, 5, 6];
    let index = take_fifth(new_vec); // Option<i32>

    // Some(number)
    if index.is_some() {
        // bool
        // Option<i32>
        println!("I got a number: {}", index.unwrap());
    }
}
