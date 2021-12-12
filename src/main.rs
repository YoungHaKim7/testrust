// Vecs

fn main() {
    let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // Everthing is the same as above except we added vec!.
    let three_to_five = &vec_of_ten[2..5];
    let start_at_two = &vec_of_ten[1..];
    let end_at_five = &vec_of_ten[..5];
    let everything = &vec_of_ten[..];

    println!(
        " Three to five: {:?},\n start at two: {:?},\n end at five: {:?},\n everything: {:?}",
        three_to_five, start_at_two, end_at_five, everything
    );
}
