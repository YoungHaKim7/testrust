// STRUCTS =   your own type, your own data structure

struct FileDirectory; // Unit struct

struct Colour(u8, u8, u8); // tuple struct

struct SizeAndColour {
    size: u32,
    colour: Colour,
}
// named struct

fn main() {
    let my_colour = Colour(50, 0, 50);

    let size_and_colour = SizeAndColour {
        size: 150,
        colour: my_colour,
    };
}
