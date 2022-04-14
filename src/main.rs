// chars
// .escape_unicode

fn main() {
    let korean_word = "청춘예찬";
    for c in korean_word.chars() {
        print!("{} ", c.escape_unicode());
    }
}

