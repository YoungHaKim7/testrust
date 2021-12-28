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

    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
        println!("Changed to dog! Now I am: {:?}", self);
    }

    fn change_to_cat(&mut self) {
        self.animal_type = AnimalType::Cat;
        println!("Changed to cat! Now I am: {:?}", self);
    }
}

fn main() {
    let mut my_animal = Animal::new_dog(10);
    my_animal.print(); // syntactic sugar
    my_animal.change_to_cat();
    my_animal.change_to_dog();
}
