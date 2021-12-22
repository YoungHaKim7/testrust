// struct = and
// enum = or     enum=enumeration e=from   number

enum ThingsInTheSky {
    Sun,   //0
    Stars, // 1
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Stars,
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun"),
        ThingsInTheSky::Stars => println!("I can see the stars"),
    }
}

fn main() {
    check_skystate(&create_skystate(20));
}
