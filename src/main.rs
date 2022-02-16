fn returns_reference<'a>() -> &str {
    let my_string = "David".to_string(); // &'static - for the life of the program
    &my_string
}

fn main() {}
