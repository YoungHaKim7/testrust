enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

fn main() {
    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter]; //Vec<Season>
    for season in four_seasons {
        println!("The number is: {}", season as u32);
    }
}
