// .then

fn main() { // bool -> Option<T>
println!("{:?}", true.then(|| {
    9
}));
}
