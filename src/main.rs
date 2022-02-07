// ('a'..='i').collect
// any 여러개 중에 한개만 맞아도 true
// all true / 다 맞아야 true

fn main() {
    let char_vec = ('a'..'監').collect::<Vec<char>>();
    println!("{}", char_vec.iter().count());
}
