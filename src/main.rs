// map
// for_each

fn main() {
    let num_vec = vec![2,4,6];

    num_vec
        .iter() // 2, 4, 6
        .enumerate() // (0,2), (1,4), (2,6)
        .for_each(|tuple| { // ()
            println!("The number at index {} is {}", tuple.0, tuple.1);
        });
}