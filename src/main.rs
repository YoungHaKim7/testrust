// .zip 
// 주의할점 iterator여 한다.!! .into_iter()한번 쓰면 사라지는 기능
use std::collections::HashMap;

fn main() {
    let some_numbers = vec![0,1,2,3,4,5,]; // a Vec<i32>
    let some_words = vec!["zero", "one","two","three","four"]; //Vec<&str> 4까지만 연결되고 나머지는 버려짐 2개가 다르기 때문

    let number_word_hashmap: HashMap<i32, &str> = some_numbers
        .into_iter()
        .zip(some_words.into_iter())
        .collect();

    let result_str = number_word_hashmap.get(&10).unwrap_or_else(||{
        println!("Help");
        &"no number"
    });
    
    println!("{result_str}");
}