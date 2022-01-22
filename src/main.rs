// blanket trait implementtations
// implementing a trait for every type that want to have it
use std::fmt::Debug;

trait Prints: Debug {
    fn prints_something(&self) {
        println!("I am: {:?}", self);
    }
}

#[derive(Debug)]
struct Person;
struct Building;

impl<T: Debug> Prints for T {}

fn main() {
    let my_person = Person;
    let my_building = Building;
    my_person.prints_something();
}
