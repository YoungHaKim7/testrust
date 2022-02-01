// .zip
use std::collections::HashMap;

fn main() {
    let some_numbers = vec![0,1,2,3,4,5,]; // a Vec<i32>
    let some_words = vec!["zero", "one","two","three","four","five"]; //Vec<&str>

    let number_word_hashmap: HashMap<i32, &str> = some_numbers
        .into_iter()
        .zip(some_words.into_iter())
        .collect();

    let result_str = number_word_hashmap.get(&10).unwrap_or_else(||{
        &"no number"
    });

    println!("{result_str}");
}