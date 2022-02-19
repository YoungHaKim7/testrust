// interior mutability
// chaging on the inside

// & immutable reference -> 정확한 표현은shared reference 여러가지를 쓸 수 있음
// &mut mutable reference -> 정확한 표현은 unique reference 딱 한개만 쓸 수 있다.

// 이거외에 다른게 쓸려고 만들게 cell 이걸 쓰면 mut를 안 써도 바꿀 수 있다.
// Cell - small room

// cell 외에
// RefCell
// Mutex
// RwLock

// not thread safe
// let mut x = 9; // 11
// thread 1 {
//     x +=1;
// }
// thread 2 {
//     x +=1;
// }

fn main() {}
