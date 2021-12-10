// OWNERSHIP - 소유권

// & = reference

fn main() {
    let country = String::from("대한민국");
    let ref_one = &country;
    let ref_two = &country;

    println!("Country is : {}", ref_one);
}
