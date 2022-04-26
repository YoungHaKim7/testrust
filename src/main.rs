// Programming Rust
// Perhaps after reading this chapter you've decided that you hate macros.

macro_rules! print_anything {
    ($($input:tt),+) => {
        let output = stringify!($input);
    };
}

fn main() {
    print_anything!(thothe, 9, thdoefgoe, 765);
}
