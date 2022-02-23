// Rc
use std::rc::Rc;

fn takes_a_string(input: String) {

}

fn also_takes_a_string(input: String) {

}

fn main() {
    let my_string = "Hello there".to_string();
    takes_a_string(my_string);
    also_takes_a_string(my_string);
}
