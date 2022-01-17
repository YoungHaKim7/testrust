// simple trait
struct Animal {
    name: String,
}

trait Canine {
    // dog-like
    fn bark(&self) {
        println!("woof woof");
    }
    fn run(&self) {
        println!("I am running!")
    }
}

impl Canine for Animal {}

fn main() {
    let my_animal = Animal {
        name: "Mr. Mantle".to_string(),
    };

    my_animal.bark();
    my_animal.run();
}
