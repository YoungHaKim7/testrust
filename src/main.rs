// Collection types: arrays, vectors (Vecs)
// ARRAYS     [ ] all the same type
fn main() {
    let array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let three_to_five = &array_of_ten[2..5];
    let start_at_two = &array_of_ten[1..5];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];

    println!(
        " Three to five: {:?},\n start at two: {:?},\n end at five: {:?},\n everything: {:?}",
        three_to_five, start_at_two, end_at_five, everything
    );
}
