// filter
// filter_map

fn main() {
    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let filtered_months = months
        .into_iter()
        .filter(|month| month.len() < 5)
        .filter(|monnnth| monnnth.contains("u"))
        .collect::<Vec<&str>>();

    println!("{:?}", filtered_months);
}
