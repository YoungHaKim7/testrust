// Other collection types
// HashMap, BtreeMap    HashMap은 python의 Dictionary와 비슷함

// Key, Value
// Key: String
// Value : Vec<String>
// land: 나라, 국가

// HashMap<String, Vec<String>>
use std::collections::HashMap;

struct City {
    name: String,
    population: HashMap<u32, u32>, // year + population
}

fn main() {
    let mut tallin = City {
        name: "Tallinn".to_string(),
        population: HashMap::new(),
    };

    tallin.population.insert(1372, 3_250);
    tallin.population.insert(1851, 24_000);
    tallin.population.insert(2020, 437_619);

    for (year, population) in tallin.population {
        println!("In the year {} the population was {}", year, population);
    }
}
