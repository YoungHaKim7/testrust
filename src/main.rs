// references and the dot operator

fn main() {
    let my_number = 10; // i32
    let references = &my_number; // &i32

    println!("Are they the same? {}", my_number == *references);
}
