// struct = and 이거 들어있고 들어 있고 할때!!
// Rust에는 struct가 3가지가 있다.
//      1. unit struct - 아무것도 없고 이름만 씀 -"byte는 0이다."
//      2.  struct
//      3.  struct
// enum = or 선택해서 쓸 때 씀.... struct와 약간 비슷

struct FileDirectory;

fn takes_file_directory(input: FileDirectory) {
    println!("I got a file directory");
}

fn main() {
    let x = FileDirectory;
    println!("The size is {}", std::mem::size_of_val(&x));
}
