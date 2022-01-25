// Chaining methods and functional style
fn main() {
    let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let new_vec = my_vec
        .into_iter()
        .skip(1)
        .skip(1)
        .skip(1)
        .skip(1)
        .take(4)
        .collect::<Vec<i32>>();

    println!("{new_vec:?}");
}
