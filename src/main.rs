fn main() {
    let mut my_vec = vec!["sun", "moon", "sun", "moon", "sun", "moon", "moon"];
    my_vec.dedup();
    println!("{my_vec:?}");
}
