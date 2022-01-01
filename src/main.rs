fn print_and_give_item() -> i32 {
    let number = 9;
    println!("The number is : {}", number);
    9
}

//generics(i32, String)   반대 개념은 concrete
// 외국말 중 노래 중에 특성이 없고 재미없는 노래를 It's a little generic 이라고 함.

fn main() {
    let x = print_and_give_item();
}
