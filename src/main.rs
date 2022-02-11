// skip, take, fold
// fold는 숫자부터 연습하면서 감각을 키워야한다.!!
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
