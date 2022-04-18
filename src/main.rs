// dedup - deduplicate
#![feature(slice_partition_dedup)]
fn main() {
    let mut slice = ["foo", "Foo", "BAZ", "Bar", "bar", "baz", "BAZ"];

    let (dedup, duplicates) = slice.partition_dedup_by(|a, b| a.eq_ignore_ascii_case(b));

    println!("{:?}", dedup);
    println!("{:?}", duplicates);
}
