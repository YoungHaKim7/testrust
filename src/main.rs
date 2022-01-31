// map
// for_each

fn main() {
    let num_vec = vec![2,4,6];

    num_vec
        .iter() // 2, 4, 6
        .enumerate() // (0,2), (1,4), (2,6)
        .for_each(|(index, number)| { // ()
            println!("The number at index {index} is {number}");
        });
}