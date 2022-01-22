// blanket trait implementtations
// implementing a trait for every type that want to have it

trait Prints {
    fn prints_something(&self) {
        println!("I like to print things");
    }
}

struct Person;
struct Building;

impl Prints for Person {}

fn main() {
    let my_person = Person;
    let my_building = Building;
    my_person.prints_something();
}
