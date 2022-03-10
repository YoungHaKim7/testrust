// Fn 
// FnMut 
// FnOnce
//

fn main () {
    let my_string = String::from("Hello there");

    let print_it = || {
        drop(my_string);
        // my_string을 drop 해서 밑으로 안 내려감.
    };

    print_it();
}
