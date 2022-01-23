trait PrintSomething {
    fn print_something(&self) {
        println!("I like to do stuff");
    }
}
struct Person;
struct Building;

impl<T> PrintSomething for T {}

// implement trait for all types that you want to have it
// blanket trait implementtation

fn main() {
    let person = Person;
    let building = Building;
    person.print_something();
    building.print_something();
}
