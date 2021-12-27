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
    fn new_cat(age: u8) -> Self {
        // Self = Animal
        Self {
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }
}

fn main() {
    let my_animal = Animal::new_cat(10);

    println!("I made a : {:?}", my_animal);
}
