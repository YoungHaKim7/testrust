// 인기가 많은 vec!
// 0
// Vec<u8>
// reallocation

fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");

    let my_vec = vec![name1, name2];

    println!("My cats are {:?}", my_vec);
}
