use rand::prelude::*;
// Note: talk about prelude

// prelude 만드는 방법
// mod prelude {
//     pub use TypeName;
// }

fn main() {
    let mut random_generator = rand::thread_rng();
    for _ in 0..40000 {
        let bigger_character =
            char::try_from(random_generator.gen_range(u32::MIN..u32::MAX)).unwrap_or('-');
        print!("{bigger_character}");
    }
}

