// ok - Result to Option
// ok_or - Option to Result
// ok_or_else - Option to Result with closure  else가 나오면 closure가 있는거다.!! 암기!!

struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        // ""
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        };

        Self {
            name: name.to_string(),
            ceo,
        }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

fn main() {
    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Doug Suttles"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];

    let mut results_vec = vec![];

    company_vec.iter().for_each(|company| {
        results_vec.push(company.get_ceo().ok_or("No CEO found"));
    });

    println!("{results_vec:?}");
}
