// dedup - deduplicate
fn main() {
    let mut my_vec = vec![10; 10000];
    println!("{}", my_vec.capacity());
    my_vec.push(9);

    println!("{}", my_vec.capacity());
}
