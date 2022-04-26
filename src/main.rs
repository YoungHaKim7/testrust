// Programming Rust
// Perhaps after reading this chapter you've decided that you hate macros.

macro_rules! make_a_function {
    ($name:ident, $($input:tt),*) => {
        fn $name() {
            let output = stringify!($($input),*);
            println!("{output}");
        }
    };
}

fn main() {
    make_a_function!(print_it, 6, 7, 8, "I");
    make_a_function!(say_its_nice, this, is, really, nice);
    print_it();
    say_its_nice();
}
