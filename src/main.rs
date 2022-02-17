// <'_> lifetime이 정해져 있다는 약속이다.syntax
struct Adventurer<'a> {
    name: &'a str,
    hit_points: u32,
}

// implicit == not said
// elided == not shown
// 어떤 lifetime인지 미리 말을 해줘야 하는데 말을 안해 줘서 에러남.
impl Adventurer<'_> {
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!("{} has{} hit hit points left!", self.name, self.hit_points);
    }
}

fn main() {}
