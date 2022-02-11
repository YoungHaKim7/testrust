// skip, take, fold
fn main() {
    let a_string = "I don't have any dashes in me.";

    let dashed = a_string
        .chars() // iterator
        .fold("-".to_string(), |mut string_so_far, next_char| {
            string_so_far.push(next_char); // ()
            string_so_far.push('-'); // ()
            string_so_far
        });
    println!("{dashed}");
}
