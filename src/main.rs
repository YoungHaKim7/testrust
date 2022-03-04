// Mutex and RwLock
use std::sync::{Mutex, RwLock}; // Read Write (Rw)

fn main () {
    let my_mutex = Mutex::new(5);

    let mut mutex_changer = my_mutex.lock().unwrap();
    let mut other_mutex_changer = my_mutex.try_lock();
    if let Ok(value) = other_mutex_changer {
        println!("The mutex_changer has: {value}");
    }
        else {
        println!("Didn't get a lock");
        }

    println!("{my_mutex:?}");
}
