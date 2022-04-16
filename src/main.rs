// .then

fn main() { // bool -> Option<T>
println!("{:?}", false.then(|| {
    9
}));
}
