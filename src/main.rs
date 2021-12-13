// Array(가장 빠름), Vec(약간 느리지만 사용성 높고 편리하게 쓸수 있음)
// &str, String
// Vec<u8>
// Vec<String>

fn main() {
    let seasons = ["봄", "여름", "가을", "겨울", "봄", "여름", "가을", "겨울"];
    println!("{:?}", &seasons[..=4]); // up to and including
}
