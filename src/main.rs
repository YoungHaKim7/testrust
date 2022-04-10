use thiserror::Error;

#[derive(Debug)]
struct User {
    points: u32,
    age: u8,
}

impl User {
    fn try_new(age: u8, points: u32) -> Result<Self, CompanyError> {
        use CompanyError::*;
        match (age, points) {
            (age, points) if age > 120 && points > 10000 => {
                Err(TooBigAndTooOld(User { age, points }))
            }
            (_, p) if p > 10000 => Err(TooBig(p)),
            (a, _) if age > 120 => Err(TooOld(a)),
            _ => Ok(Self { age, points }),
        }
    }
}

#[derive(Error, Debug)]
enum CompanyError {
    #[error("Not enough data")]
    NotEnoughtData,
    #[error("Too old: {0} Can't be over 120")]
    TooOld(u8),
    #[error("God {0}, should be under 10,000")]
    TooBig(u32),
    #[error("Must be under 120 and 10,000 points, got{0:?} instead")]
    TooBigAndTooOld(User), // UserBuilder
}

fn main() {
    let user_requests = vec![
        User::try_new(150, 20000),
        User::try_new(100, 20000),
        User::try_new(250, 1000),
        User::try_new(40, 5000),
    ];

    println!("{user_requests:?}");
}
