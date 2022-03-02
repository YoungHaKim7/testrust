use std::cell::RefCell;

trait CoolTrait {
    fn cool_function(&self);
}


struct OurStruct {
    data: RefCell<u8>
}

impl CoolTrait for OurStruct {
    fn cool_function(&self) {
        *self.data.borrow_mut() += 1;
    }
}

fn main () {}
