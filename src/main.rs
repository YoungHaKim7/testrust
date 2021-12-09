fn main() {
    // String
    // &str ref str "string slice"
    let my_name = "David"; // &str
    let my_name1 = "David".to_string(); // String
    let other_name = String::from("David2");
    // growable + shrinkable
    let mut my_other_name = "David3".to_string();
    my_other_name.push('!');
    println!("{}", my_other_name);
}
