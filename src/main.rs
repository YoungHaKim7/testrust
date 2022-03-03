use std::sync::{Arc, Mutex};
use std::thread;

// Arc<Mutex>
// atomic reference counter
// operating system primitives

trait CoolTrait {
    fn cool_function(&self);
}

struct OurStruct {
    data: Arc<Mutex<u8>>,
}

impl CoolTrait for OurStruct {
    fn cool_function(&self) {
        *self.data.lock().unwrap() += 1;
    }
}

fn main() {
    let our_struct = OurStruct {
        data: Arc::new(Mutex::new(0)),
    };

    let mut join_vec = vec![];

    for _ in 0..10 {
        let clone = Arc::clone(&our_struct.data); // Arc<Mutex<u8>>
        let join_handle = thread::spawn(move || {
            // move = take by value
            *clone.lock().unwrap() += 1;
        });
        join_vec.push(join_handle);
    }

    for handle in join_vec {
        handle.join().unwrap();
    }
}
