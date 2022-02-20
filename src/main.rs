// panic의 정의 : 프로그램을 그만 하는거 , 어떤 방에 들어갈을 때 이 방은 위험한것 같아 하면서 그만두는거임.
// 위의 정의를 unwind the stack, OS에게 프로그램을 돌려주는거임.!!
// RefCell 정의 : runtime checked borrwing rules
use std::cell::RefCell;

// unwind the stack
#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
    // Many other fields
}

fn main() {
    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };
}
