// type generics - generics over a type
// const generics - generics over a const
struct SomeArrays<T> {
    array_1: [T; 3],
    array_2: [T; 4],
}

fn main() {
    let my_arrays = SomeArrays {
        array_1: [8, 9, 10],
        array_2: [89, 98, 98, 99],
    };
}
