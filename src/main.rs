// naive implementation

#[derive(Debug)]
struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: Lifestate,
    can_use: bool, // flag
}

#[derive(Debug)]
enum Lifestate {
    Alive,
    Dead,
    NeverAlice,
    Uncertain,
}

fn use_character(character: &Character) {
    if character.can_use {}
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Billy".to_string(),
            age: 15,
            height: 170,
            weight: 70,
            lifestate: Lifestate::Alive,
            can_use: true,
        }
    }
}

impl Character {
    fn with_age(mut self, age: u8) -> Self {
        self.age = age;
        self.can_use = false;
        self
    }
    fn with_weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self.can_use = false;
        self
    }
    fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self.can_use = false;
        self
    }
    fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self.can_use = false;
        self
    }
    fn build(mut self) -> Result<Character, String> {
        // Smurf smurf SMuRf
        if self.height < 200 && self.weight < 300 && !self.name.to_lowercase().contains("smurf") {
            self.can_use = true;
            Ok(self)
        } else {
            Err("Names must not contain Smurf, weight must be....".to_string())
        }
    }
}

fn main() {
    let npc_1 = Character::default()
        .with_age(20)
        .with_height(194)
        .with_weight(98)
        .with_name("Billybrobby")
        .build();
    println!("{npc_1:?}");
}
