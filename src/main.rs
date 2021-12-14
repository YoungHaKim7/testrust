// Vec<String>
// Vec<u8>
// T = some type
// generics

fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");

    let mut my_vec = Vec::new();

    my_vec.push(name1);
    my_vec.push(name2);

    println!("My cats are {:?}", my_vec);
}
