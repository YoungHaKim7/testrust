// retain
// String pop
// from_utf8
// from_utf8_lossy
// split_off

fn main() {
    let mut my_vec = vec![8, 9, 10, 11]; // .iter().filter()
    my_vec.retain(|number| number > &9);
    println!("{my_vec:?}");
}
