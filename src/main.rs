trait Booky {}

struct Book;
struct BigBook;

impl Booky for Book {}
impl Booky for BigBook {}

// dyn = dynamically decided = runtime에서 결정함.
fn main () {
    let vec_of_booky_things: Vec<Box<dyn Booky>> = vec![Box::new(Book), Box::new(BigBook)];
    }
