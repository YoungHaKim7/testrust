use core::time;
use std::cell::{Cell, RefCell};

// Rc = ownership 원래 러스트의 소유권자는 1개인데
// Rc 는 소유권자가 4,5,6,7 이렇게 많이 늘어남. -- 뭐지..
// 소유권자가 5,4, 3, 이렇게 하다가 0으로 되면 그때 drop 됨

// drop RC<dyn Any> dyn는  runtime
// Rc - reference counter
// 레퍼런스 카운더는 파이썬 같이 아무 타입이 들어가서 소유권자가 2,1 0으로 떨어질 때 drop
// Rc<Cell>, Rc<RefCell>, 

// Arc = thread safe Reference Conter 다른 쓰레드에 들어갈 수 있다.!
// Arc<Mutex> = Rc<Cell>과 비슷하다.! thread safe 
// Arc<Mutex> // Atomic reference counter 쓰레드를 여러개를 쓰고 있고 value를 바꿔야 할 때 쓴다.

// Interior mutability
// extern-crate tokio rocket rand
trait SuperCoolTrait {
    fn cool_function(&self);
}

#[derive(Debug)]
struct User {
    id: u32,
    times_used: Cell<u32>,
}

impl SuperCoolTrait for User {
    fn cool_function(&self) {
        println!("Now using cool_function");
        let times_used = self.times_used.get();
        self.times_used.set(times_used + 1);
    }
}
fn main() {
    let user = User {
        id: 89723987,
        times_used: Cell::new(0),
    };

    for _ in 0..20 {
        user.cool_function();
    }

    println!("{user:?}");
}
