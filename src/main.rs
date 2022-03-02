// interior mutability
// multiple threads
// Rc<RefCell>

trait CoolTrait {
    fn cool_function(&self);
}


struct OurStruct {
    data: u8
}

impl CoolTrait for OurStruct {
    fn cool_funtion(&self) {
        self.data += 1;
    }
}

fn main () {}
