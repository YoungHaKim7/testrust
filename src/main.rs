// Vec<(&str, i32)>
// Destructuring
// Structure

fn main() {
    let str_array = ["one", "two", "three"];

    let [a, _, _] = str_array;

    println!("Item a is : {}", a);
}
