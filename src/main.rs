// .then

fn main() { // bool -> Option<T>
    let bool_vec = vec![true, false,true, false, false];

    let option_vec = bool_vec
        .iter()
        .map(|item| {
            item.then(|| {
                println!("Got a {item}!");
                "It's true, you know" // Option<&str>
            })
        })
        .collect::<Vec<_>>();
    println!("{option_vec:?}");
}
