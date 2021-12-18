fn main() {
    let children = 5;
    let married = true;

    match (children, married) {
        (c, m) if married == false => println!("Not married with {}", children),
        // false 표시는 !married
        // true 표시는 marred로 표현할 수 있다.!!
        (c, m) if c == 0 && m => println!("Married but with no children"),
        _ => println!("Some other type of marriage and children combination"),
    }
}
