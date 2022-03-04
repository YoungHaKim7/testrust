//
// Cell 젤 간단하고 젤 빠름 multiful thread 가 필요 없을 경우 사용하면 된다.
//
// RefCell -> Thread safe (x)ㄱㅏ 아님 (더 빠름)
// Mutex -> Thread safe (o)
//
// Rc - Thread safe (x) Rc 가 더 빠름
// Arc - > Thread safe (o)
// Arc has more overhead  뭔가 더 복잡하다는 뜻
//
// Mutex and RwLock
use std::sync::{Mutex, RwLock}; // Read Write (Rw)

fn main () {
    let my_mutex = Mutex::new(5);

    let mut mutex_changer = my_mutex.lock().unwrap();

    println!("{my_mutex:?}");
}
