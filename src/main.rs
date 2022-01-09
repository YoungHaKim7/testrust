use std::collections::HashMap;

fn main() {
    // Key -> Value
    let mut book_hashmap = HashMap::new();

    book_hashmap.insert(1, "L'Allemagne Moderne");

    if book_hashmap.get(&1).is_none() {
        book_hashmap.insert(1, "Le Petit Prince");
    } else {
        println!("Already got a book");
    }
}
