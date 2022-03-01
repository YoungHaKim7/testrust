use std::thread;

fn main() {
    let join_handle = thread::spawn(|| {
        println!("I am printing something");
    });

    join_handle.join(); // wait
}



