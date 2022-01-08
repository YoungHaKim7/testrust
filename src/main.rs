// BTreeMap<String, Vec<String>>
// HashMap은 출력할때 랜던하는 순서로 나오지만 BtreeMap은 입력한 순서대로 출력 가능!!
use std::collections::BTreeMap;

struct City {
    name: String,
    population: BTreeMap<u32, u32>, // year + population
}

fn main() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(),
    };

    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_619);

    for (year, population) in tallinn.population {
        println!("In the year {} the population was {}", year, population);
    }
}
