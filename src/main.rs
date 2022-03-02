use std::cell::RefCell;
use std::thread;

trait CoolTrait {
    fn cool_function(&self);
}

struct OurStruct {
    data: RefCell<u8>,
}

impl CoolTrait for OurStruct {
    fn cool_function(&self) {
        *self.data.borrow_mut() += 1;
    }
}

fn main() {
    let our_struct = OurStruct {
        data: RefCell::new(0)
    };

    let mut join_vec = vec![];
    for _ in 0..10 {
        let join_handle = thread::spawn(|| {
            *our_struct.data.borrow_mut() += 1;
        });
        join_vec.push(join_handle);
    }

    for handle in join_vec {
        handle.join().unwrap();
    }
}
