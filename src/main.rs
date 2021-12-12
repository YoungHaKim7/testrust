// TUPLES

fn main() {
    let random_tuple = (7, 8, "This is a str", [7, 8, 9], vec![9, 8], 7.8);
    let random_tuple_2 = (7, 8);

    println!("{}", random_tuple.1 + random_tuple_2.0);
}
