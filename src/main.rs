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

//  // impl = implement
// impl Animal {
//     fn new() ->
// }

fn main() {
    let my_animal = Animal {
        age: 10,
        animal_type: AnimalType::Cat,
    };
}
