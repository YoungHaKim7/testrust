fn main() {
    // reallocation

    // String
    // .capacity
    // .push
    // .push_str
    // .pop
    // .with_capacity

    // method = .Function
    let mut my_name = "".to_string();
    println!(
        "Length is {} and capacity is : {}",
        my_name.len(),
        my_name.capacity()
    );
    my_name.push_str("David!");
    println!(
        "Length is {} and capacity is : {}",
        my_name.len(),
        my_name.capacity()
    );
    my_name.push('!');
    my_name.push_str(" and I live in Seoul");
    println!(
        "Length is {} and capacity is : {}",
        my_name.len(),
        my_name.capacity()
    );
}
