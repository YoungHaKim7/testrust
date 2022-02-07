// ('a'..='i').collect
// any 여러개 중에 한개만 맞아도 true
// all true / 다 맞아야 true

fn in_char_vec(char_vec: &Vec<char>, check: char) {
    println!(
        "Is {} inside?",
        char_vec.iter().any(|&character| character == check)
    )
}

fn main() {
    let char_vec = ('a'..'監').collect::<Vec<char>>();
    in_char_vec(&char_vec, 'i');
    in_char_vec(&char_vec, '붹');
    in_char_vec(&char_vec, '無');
}
