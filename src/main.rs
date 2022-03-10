// Fn (&Self)
// FnMut (&mut self)
// FnOnce
//

fn main () {
    let my_string = String::from("Hello there");

    let print_it = || {
        println!("{my_string}");
    };

    print_it();
}
