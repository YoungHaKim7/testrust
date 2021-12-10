// OWNERSHIP - 소유권

fn return_it() -> &'static String {
    let country = String::from("대한민국");
    &country // return &String
}

// & = reference

fn main() {
    let my_country = return_it();
}
