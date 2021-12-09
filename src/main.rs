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
    println!("Capacity is : {}", my_name.capacity());
    my_name.push_str("David!");
    println!("Capacity is : {}", my_name.capacity());
    my_name.push('!');
    my_name.push_str(" and I live in Seoul");
    println!("Capacity is : {}", my_name.capacity());
}
