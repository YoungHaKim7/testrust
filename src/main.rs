macro_rules! check {
    ($input1: ident, $input2:expr) => {
        println!(
            "Is {:?} equal to {:?}? {}",
            $input1,
            $input2,
            $input1 == $input2
        );
    };
}

fn main() {
    let x = 8;
    check!(x, 9);
}
