// Vec<(&str, i32)>
// Destructuring
// Structure

fn main() {
    let str_tuple = ("one", "two", "three");

    let (a, _, _) = str_tuple;

    println!("Item a is : {}", a);
}
