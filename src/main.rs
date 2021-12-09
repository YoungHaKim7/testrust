fn give_number(one: i32, two: i32) -> i32 {
    let mutiplied_by_ten = {
        let first_number = 10;
        first_number * one * two
    };
    mutiplied_by_ten
}

fn main() {
    let my_number = give_number(9, 1);
    println!("{}", my_number);
}
