trait Booky {}

struct Book {
    name: String,
}
struct BigBook;

struct City {
    year_founded: i32
}
impl Booky for Book {}
impl Booky for BigBook {}
impl Booky for City {}

// dyn = dynamically decided = runtime에서 결정함.
// Generics = Compile 타임에서 하는것 
fn main() {
    let my_city = City {
        year_founded: 1989
    };

    let vec_of_booky_things: Vec<Box<dyn Booky>> = vec![
        Box::new(Book {
            name: "my_book".to_string(),
        }),
        Box::new(BigBook),
        Box::new(my_city)
    ];
}
