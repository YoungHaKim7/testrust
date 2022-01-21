#[derive(Debug)] // So we can print City
struct City {
    name: String,
    population: u32,
}

#[derive(Debug)] // Country also needs to be printed
struct Country {
    cities: Vec<City>, //Our cities go in here
}

impl City {
    fn new(name: &str, population: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
        }
    }
}

// Country::from(vec![City, City])
impl From<Vec<City>> for Country {
    fn from(cities: Vec<City>) -> Self {
        Self { cities }
    }
}

impl Country {
    fn print_cities(&self) {
        for city in &self.cities {
            println!("{:?} has a population of {:?}", city.name, city.population);
        }
    }
}

fn main() {
    let helsinki = City::new("Helsinki", 631_695);
    let turku = City::new("Turku", 186_756);

    let finland_cities = vec![helsinki, turku];
    let finland: Country = finland_cities.into();
    finland.print_cities();

    //.into()   From이랑 똑같지만 복잡하게 쭉 쓰고 마지막에 .into하고 만들고 싶은 타입을 입력해줌
    // let x = .....iter().for_each().into()
    // 단독으로 into를 쓸수 없다. 이유는 타입이 정해져 있지 않기 때문이다.__rust_force_expr!
    // from 을 쓰고 into로 연결해 준다.
}
