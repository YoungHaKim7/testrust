struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: Lifestate,
}

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

fn main() {
    let npc_1 = Character::new("Billy".to_string(), 15, 170, 70, true);
}

