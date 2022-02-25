#[derive(Debug)]
struct DataContainer<'a> {
    data: &'a mut String
}

fn main() {
    let mut important_data = "Super duper important data".to_string();

    let mut container_1 = DataContainer {
        data: &mut important_data
    };

    let mut container_2 = DataContainer {
        data: &mut important_data
    };

    for _ in 0..10 {
        *container_1.data = String::from("Hi");
        *container_2.data = String::from("Hi there!");
    }
}
