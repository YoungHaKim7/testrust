// Slices
// Vecs

// dynamically sized type

fn main() {
    let seasons = ["봄", "여름", "가을", "겨울", "봄", "여름", "가을", "겨울"];
    println!("{:?}", &seasons[..=4]); // up to and including
}
