trait Booky {}

struct Book;
struct BigBook;

impl Booky for Book {}
impl Booky for BigBook {}

fn main () {
    let vec_of_booky_things: Vec<Booky> = vec![Book, BigBook];
    }
