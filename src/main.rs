// Dafault and the builder pattern

struct SomeStruct;

fn main() {
    let my_struct = SomeStruct {
        name: c7987,
        population: 9879869
    };

    let my_struct = SomeStruct::default().with_name("some name").with_population(98263987623).build();
}
