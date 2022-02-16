fn returns_reference() -> &str {
    let my_string = "David".to_string(); // &'static - for the life of the program
    let my_string_ref = &my_string; // &str - reference to something else
}

fn main() {}
