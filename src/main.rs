// column!()
// line!()
// line!()
// module_path!()

fn main() {
    println!("{} {} {} {}", column!(), line!(), file!(), module_path!());

    let my_file = include_str!(
        "/Users/globalyoung/Documents/Project/Github/rust_project/testrust/txt_sort/rust_sort.txt"
    );

    println!("{:?}", include_bytes!("/Users/globalyoung/Documents/Project/Github/rust_project/testrust/txt_sort/rust_sort.txt"
    ));
}

