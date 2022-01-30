
fn main () {
    let my_vec = vec![];

    let fourth = my_vec.get(3).unwrap_or(&9);

    println!("{fourth}");

}