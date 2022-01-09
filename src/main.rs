use std::collections::HashMap;

fn main() {
    // Key -> Value
    let mut book_hashmap = HashMap::new();

    book_hashmap.insert(1, "L'Allemagne Moderne");

    if let Some(book_name) = book_hashmap.get(&1) {
        println!("Already got a book");
    } else {
        book_hashmap.insert(1, "Le Petit Prince");
    }
}
