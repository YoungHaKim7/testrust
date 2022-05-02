fn main() {
    let rng = rand::thread_rng();

    println!("Hello, world");

    let my_string = String::from("I am String");

    let my_vec = vec![8, 9, 10];

    let my_new_vec = my_vec
        .iter() // iterators are lazy
        .map(|number| number + 1)
        .collect::<Vec<i32>>();
}
