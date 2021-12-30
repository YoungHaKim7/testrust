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
    fn new(age: u8, animal_type: AnimalType) -> Self {
        Self { age, animal_type }
    }

    fn check_type(&self) {
        use AnimalType::*;
        match self.animal_type {
            Cat => println!("Animal type is cat"),
            Dog => println!("Animal type is dog"),
        }
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
    use AnimalType::*;
    let my_cat = Animal::new(10, Cat);
    let my_dog = Animal::new(10, Dog);

    my_cat.check_type();
}
