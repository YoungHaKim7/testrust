// dedup - deduplicate
fn main() {
    let mut my_vec = vec![10; 10000];
    println!("{}", my_vec.capacity()); // Vec::with_capacity(10001);
    my_vec.push(9);

    println!("{}", my_vec.capacity());
    my_vec.shrink_to_fit();
    println!("{}", my_vec.capacity());
}
