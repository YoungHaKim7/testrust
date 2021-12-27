#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

// impl = implement
impl Animal {
    // function signature
    fn new() -> Self {
        // Self = Animal
        Self {
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }
}

fn main() {
    let my_animal = Animal::new(); // associated function
}
