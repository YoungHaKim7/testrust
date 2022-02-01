fn main() {
    let num_vec = vec![2,4,6];

    let new_thing = num_vec
        .iter() // 2, 4, 6
        .enumerate() // (0,2), (1,4), (2,6)
        .map(|(index, number)| {
            println!("The number at index {index} is {number}");
            0
        })
        .collect::<Vec<_>>();

        println!("{new_thing:?}");
}