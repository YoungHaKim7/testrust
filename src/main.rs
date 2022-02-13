// match_indices indices = indexes
// peekable

fn user_input() -> String {
    // do something
    "user_name".to_string()
}

fn main() {
    // enuberate
    let rules = "Rule number 1 : No fighting.
        Rule number 2 : Goto bed at 8 pm.
        Rule number 3 : Wake up at 6am ";

    let rule_locations = rules.match_indices(user_input).collect::<Vec<(_, _)>>();
    println!("Rule locations: {rule_locations:?}");
}
