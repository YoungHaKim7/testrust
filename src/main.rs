#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

#[derive(Debug)]
enum AnimalType {
    Cat(String),
    Dog(String),
}

impl AnimalType {
    fn print_name(&self) {
        match self {
            AnimalType::Cat(name) => println!("Animal type is cat and name is : {}", name),
            AnimalType::Dog(name) => println!("Animal type is dog and name is : {}", name),
        }
    }
}

impl Animal {
    fn new(age: u8, animal_type: AnimalType) -> Self {
        Self { age, animal_type }
    }
}

fn main() {
    let my_cat = Animal::new(10, AnimalType::Cat("Windy".to_string()));

    my_cat.animal_type.print_name();
}
