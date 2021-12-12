// DESTRUCTURING = pulling items out to make varialbles

fn main() {
    let str_vec = vec!["one", "two", "three"];
    let (a, b, _) = (str_vec[0], str_vec[1], str_vec[2]);
    println!("{}, {}", a, b);
}
