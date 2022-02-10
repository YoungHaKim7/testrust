// skip, take, fold
fn main() {
    let some_numbers = vec![9,6,9,10,11];

    // 0
    // 0,9
    // 9 + 6
    // 15 +

    println!("{}", some_numbers
        .iter()
        .fold(0,|total_so_far, next_number|total_so_far + next_number));
}
