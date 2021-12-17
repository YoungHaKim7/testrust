// STRUCTS =   your own type, your own data structure

struct FileDirectory; // Unit struct

struct Colour(u8, u8, u8); // tuple struct

/*struct SizeAndColour {
    size: u32,
    colour: Colour,
}
 // named struct
 */

fn main() {
    let my_directory = FileDirectory;
    let some_colours = Colour(50, 60, 0);
    println!("The first colour is : {}", some_colours.0);
}
