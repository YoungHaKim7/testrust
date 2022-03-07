// trait __Dungeons and Dradons

// The focus of this talk is traits with D&D used as a metaphor
// Dwarf, Elf, Half-Orc, Human
//
// Character traits
// Strength, Intelligence, Dexterity, Wisdom, Constitution, Charisma

struct Dwarf {
    name: String
}

struct Elf {
    name: String
}

struct HalfElf {
    name: String
}

struct HalfOrc {
    name: String
}

struct Human {
    name: String
}

// All of these need to be cast
struct Cantrip {}

struct Enchantment {}

struct Transmutation {}

struct Necromancy {}

struct Spellbook {
    pub spells: Vec<Box<Cast>>,
}
// Vector contains Boxes that point to values that implement Cast trait
// Vector contains Boxes
// <T> <- Type of value Box can point to
// <Cast> <- Box : must point to a value the implement Cast trait
// Vec<T>  <- Vector that contain object of a trait
// How can we cast ALL of these?

// The constitution bonus for a dwarf is 2
impl Constitution for Dwarf {
    fn constitution_bonus(&self) -> u8 {
        2
    }
}

// The constitution bonus for a half-orc is 1
impl Constitution for HalfOrc {
    fn constitution_bonus(&self) -> u8 {
        1
    }
}

impl Constitution for Elf {}

impl Constitution for Human {}

// The constitution bonus for both a human and a half-elf is 0
// Let's make 0 the default!
pub trait Constitution {
    fn constitution_bonus(&self) -> u8 {
        0
    }
}

pub trait Elvish {}

impl Elvish for Elf {}

impl Elvish for HalfElf {}

// Let's make a function for speaking Elvish
// Accept a generic type "T"(character: T)
// Only implement the Elvish Trait<T: Elvish>
pub fn speak_elvish<T: Elvish>(character: T) -> String {
    String::from("yes")
}
// Not implemented for Half-Orc
// Trait bounds allow a function to only accept types that implement a certain trait
// Trait Objects
// You cannot add data to a trait object
// Trait objects contain both data(Pointer) and behavior(Trait)
//
// Spell in D&D (Cantrip, Transmutation, Enchantment, Necromancy)

impl Spellbook {
    pub fn run(&self) {
        for spell in self.spells.iter() {
            spell.cast(); // Cast spell 
        }
    }
}

pub trait Cast {
    fn cast(&self);
}

// Where can we keep our spells?
// In a Spellbook, of course!
// (Some Cantrip), (Special Enchantment), (A Transmutation), (Dark Necromancy)

impl Cast for Cantrip {
    fn cast(&self) {
        // Details of casting a Cantrip Spell
    }
}

impl Cast for Transmutation {
    fn cast(&self) {
        // Details of casting a Transmutation Spell
    }
}

impl Cast for Enchantment {
    fn cast(&self) {
        // Details of casting a Enchantment Spell
    }
}

impl Cast for Necromancy {
    fn cast(&self) {
        // Details of casting a Necromancy Spell
    }
}

// Trait objects are great for heterogenus collections
// It doesn't matter what type something is, as long as it implements a certain trait
// Wrapping up...
// Trait are hard..... at first
// Trait are awesome!

fn main () {
    let my_dwarf = Dwarf {
        name: String::from("NellDwarf")
    };

    let my_elf = Elf {
        name: String::from("NellElf")
    };

    let my_half_orc = HalfOrc {
        name: String::from("NellHalfOrc")
    };

    let my_human = Human {
        name: String::from("Nell")
    };
    
    let my_half_elf = HalfElf {
        name: String::from("NellElf")
    };

    let spell_book = Spellbook{
        // Different types of spells (each implements Cast)
        spells: vec![
            Box::new(Cantrip{}),
            Box::new(Transmutation{}),
            Box::new(Enchantment{}),
            Box::new(Necromancy{}),
        ],
    };

    // Casts each spells
    spell_book.run();

    // Returns 2
    my_dwarf.constitution_bonus();

    // Returns 0 (default)
    my_elf.constitution_bonus();
    
    // Returns 1
    my_half_orc.constitution_bonus();

    // Returns 0 (default)
    my_human.constitution_bonus();

    // Returns "yes"
    speak_elvish(my_elf);

    // Returns "yes"
    speak_elvish(my_half_elf);


    // Returns "yes"
    speak_elvish(my_half_orc);



}
