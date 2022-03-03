use std::sync::Mutex;
use std::thread;

// Arc<Mutex>
// Mutex
// Mutual exclusion

trait CoolTrait {
    fn cool_function(&self);
}

struct OurStruct {
    data: Mutex<u8>,
}

impl CoolTrait for OurStruct {
    fn cool_function(&self) {
        let lock = self.data.lock().unwrap();
        drop(lock);
        *self.data.lock().unwrap() += 1;
    }
}

fn main() {
    let our_struct = OurStruct {
        data: Mutex::new(0),
    };

    let mut join_vec = vec![];
    for _ in 0..10 {
        let join_handle = thread::spawn(move || {
            // move = take by value
            *our_struct.data.lock().unwrap() += 1;
        });
        join_vec.push(join_handle);
    }

    for handle in join_vec {
        handle.join().unwrap();
    }
}
