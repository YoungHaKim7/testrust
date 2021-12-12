// Vecs

fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");
    let name3 = String::from("MB");

    let mut my_vec = Vec::new();

    my_vec.push(name1);
    my_vec.push(name2);
    my_vec.push(name3);

    println!("{:?}", my_vec);
}
