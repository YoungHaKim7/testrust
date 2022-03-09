fn gives_five() -> u8 {
    5
}



fn gives_six() -> impl FnMut() {
    6
}
// closure 가 쓰는 3가지 trait
// Fn() -> u8 // reference (&self)
// FnMut() ->u8 // can mutate (&mut self)
// FnOnce() ->u8 // can be used once (self)1회용으로 끝남

fn add_to_funtion_output(my_function: fn() -> u8, some_number: u8) {
    let my_number = my_function();
    let next_number = my_number + some_number;
    println!("We got {next_number}");
}

fn main () {
    add_to_funtion_output(gives_five,8);
    add_to_funtion_output(gives_six,8);
}
