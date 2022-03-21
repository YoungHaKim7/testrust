#[derive(Debug)]
struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: Lifestate,
}

#[derive(Debug)]
enum Lifestate {
    Alive,
    Dead,
    NeverAlice,
    Uncertain,
}

impl Character {
    fn new(name: String, age: u8, height: u32, weight: u32, alive: bool) -> Self {
        Self {
            name,
            age,
            height,
            weight,
            lifestate: if alive {
                Lifestate::Alive
            } else {
                Lifestate::Dead
            },
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Billy".to_string(),
            age: 15,
            height: 170,
            weight: 70,
            lifestate: Lifestate::Alive,
        }
    }
}

impl Character {
    fn with_age(mut self, age: u8) -> Self {
        self.age =  age;
        self

    }
}

fn main() {
    let npc_1 = Character::default().with_age(20);
    println!("{npc_1:?}");
}
