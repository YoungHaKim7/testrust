#[derive(Debug)]
struct DataContainer {
    data: String
}

fn main() {
    let important_data = "Super duper important data".to_string();

    let container_1 = DataContainer {
        data: important_data
    };
}
