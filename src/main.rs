enum List {
    Content(i32, Box<List>),
    NoContent
}
// box = owned data

fn main () {
    let my_list = List::Content(8786, Box::new(List::NoContent));
    }
