// Vec<String>
// Vec<u8>
// T = some type
// generics

fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");

    let mut my_vec = Vec::new();
    println!("Space for my_vec: {}", my_vec.capacity());
    my_vec.push(name1);
    println!("Space for my_vec: {}", my_vec.capacity());
    my_vec.push(name2);
    println!("Space for my_vec: {}", my_vec.capacity());

    println!("My cats are {:?}", my_vec);
}
