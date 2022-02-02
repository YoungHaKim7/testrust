//.chars() - iterator of char
//.count() - counts nubmer of items in iterator
// char_indices
// = chars().enumerate()

// index indices(라틴어 index의 복수형)

fn main() {
    let big_string = "Hello there, I am a &str";

    big_string.char_indices().for_each(|(index, charrrrrrr)| {
        println!("At index {index} is the char {charrrrrrr}");
    });
}
