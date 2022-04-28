// type generics - generics over a type

// const generics - generics over a const

const BIG: usize = 100;

#[derive(Debug)]
struct SomeArrays<T, const N: usize> {
    array_1: [T; N],
    array_2: [T; N],
}

fn main() {
    let my_arrays = SomeArrays {
        array_1: [0; BIG],
        array_2: [0; BIG],
    };
    println!("{my_arrays:?}")
}
