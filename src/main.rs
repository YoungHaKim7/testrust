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

impl Animal {
    fn new_cat(age: u8) -> Self {
        Self {
            age,
            animal_type: AnimalType::Cat,
        }
    }
    fn new_dog(age: u8) -> Self {
        Self {
            age,
            animal_type: AnimalType::Dog,
        }
    }

    fn print(&self) {
        println!("I am a : {:?}", self);
    }
}

fn main() {
    let my_animal = Animal::new_dog(10);
    my_animal.print(); // dot operator
    Animal::print(&my_animal); // syntactic sugar
}
