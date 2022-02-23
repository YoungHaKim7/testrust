// Rc
// code 가 막혔을때 clone을 쓰지 말고 차라리 Rc를 써라
// clone은 anti-type이라고 해서 예상치 못한 오류가 생길 수 있다.
use std::rc::Rc;

fn takes_a_string(input: Rc<String>) {
    println!("{input}");
}

fn also_takes_a_string(input: Rc<String>) {
    println!("{input}");
}

fn main() {
    let my_string = Rc::new("Hello there".to_string());
    takes_a_string(Rc::clone(&my_string));
    also_takes_a_string(Rc::clone(&my_string));
    
}
