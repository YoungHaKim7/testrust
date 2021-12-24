enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar,
}

fn main() {
    use Star::*;
    let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedDwarf, DeadStar];

    for star in starvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star: {}", size),
            size if size >= 80 => println!("Pretty big star: {}", size),
            _ => println!("Some other star"),
        }
    }
    println!("What about DeadStar? It is : {}", DeadStar as u32);
}
