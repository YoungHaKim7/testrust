// map
// for_each

fn main() {
    let num_vec = vec![2,4,6];

    let double_vec = num_vec
        .iter()
        .map(|number| number *2)
        .map(|number| number *2)
        .map(|number| number *2)
        .map(|number| number *2);
        

    println!("{double_vec:?}");

}