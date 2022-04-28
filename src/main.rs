// type generics - generics over a type
// const generics - generics over a const
// const generics
// C++의 __ constexpr 와 비슷함

struct SomeVecs<T> {
    vec_1: Vec<T>,
    vec_2: Vec<T>,
}

fn main() {
    let my_vecs = SomeVecs {
        vec_1: vec![8, 9, 10],   // [i32; 3]
        vec_2: vec![89, 98, 98], // [i32; 4]
    };
}
