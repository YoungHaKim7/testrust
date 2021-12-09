// mutability
// shadowing 같은 이름을 다시 쓰는 것

fn main() {
    let my_variable = 9;
    println!("{}", my_variable);
    {
        let my_variable = "Some string";
        println!("{}", my_variable);
    }
    println!("{}", my_variable);
}
