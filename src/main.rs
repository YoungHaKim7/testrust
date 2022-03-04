// Mutex and RwLock
use std::sync::{Mutex, RwLock}; // Read Write (Rw)

fn main () {
    let my_rwlock = RwLock::new(5);

    let read1 = my_rwlock.write().unwrap(); // lock 
    let read2 = my_rwlock.try_read().unwrap();

    println!("{read1:?},{read2:?}");
}

