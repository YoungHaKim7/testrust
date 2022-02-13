// match_indices indices = indexes
// peekable

fn main() {
    // enuberate
    let rules = "Rule number 1 : No fighting.
        Rule number 2 : Goto bed at 8 pm.
        Rule number 3 : Wake up at 6am ";

    let rule_locations = rules.match_indices("Rule").collect::<Vec<(i32, &str)>>();
}
