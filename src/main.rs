use anyhow::{anyhow, Error as AnyhowError};
use serde::{Deserialize, Serialize};
use serde_json;
use thiserror::Error;
// serde
// SERialialize - turn into JSON, YAML, etc.
// DEserialize - turn into JSON, YAML, etc.

#[derive(Debug, Serialize, Deserialize)]
struct User {
    points: u32,
    age: u8,
}

// rocket, actix
#[derive(Debug, Serialize, Deserialize)]
struct UserRequest {
    points: u32,
    age: u8,
}

impl User {
    fn try_new(age: u8, points: u32) -> Result<Self, CompanyError> {
        use CompanyError::*;
        match (age, points) {
            (age, points) if age > 120 && points > 10000 => Err(TooBigAndTooOld(age, points)),
            (_, p) if p > 10000 => Err(TooBig(p)),
            (a, _) if age > 120 => Err(TooOld(a)),
            _ => Ok(Self { age, points }),
        }
    }
    fn from_request(request: UserRequest) -> Result<User, AnyhowError> {
        if request.age < 120 && request.points < 10000 {
            Ok(User {
                age: request.age,
                points: request.points,
            })
        } else {
            Err(anyhow!("User is bad")) // ; unit type으로 만들어줌
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
    #[error("Must be under 120 and 10,000 points, got{0} and {1} points instead")]
    TooBigAndTooOld(u8, u32),
}

#[derive(Error, Debug)]
#[error("I couldn't care less")]

struct DontCareError;

fn do_some_stuff(number: &str, age: u8, points: u32) -> Result<(), AnyhowError> {
    let my_number = number
        .parse::<i32>()
        .map_err(|_| anyhow!("Couldn't get a number"))?;
    let my_user = User::try_new(age, points).map_err(|_| anyhow!("Couldn't make a user"))?;
    Ok(())
}

fn main() {
    let user_requests = vec![
        User::try_new(150, 20000),
        User::try_new(100, 20000),
        User::try_new(200, 1000),
        User::try_new(40, 5000),
        User::try_new(50, 7000),
    ];

    // let users = user_requests
    //     .into_iter()
    //     .filter_map(|user_requests| match user_requests {
    //         Ok(user) => Some(user),
    //         Err(message) => {
    //             println!("{message}");
    //             None
    //         }
    //     })
    //     .collect::<Vec<User>>();

    // let try_1 = do_some_stuff("nthoetho", 30, 100);
    // let try_2 = do_some_stuff("90", 200, 100);
    // let try_3 = do_some_stuff("90", 100, 1998100);
    // let try_4 = do_some_stuff("90", 100, 100);

    // println!("{try_1:?},{try_2:?}, {try_3:?}, {try_4:?}");

    let request = r#"
    {
    "points":1000,
    "age":100
    }"#;
    let user_request: UserRequest = serde_json::from_str(request).unwrap();
    let user_try = User::from_request(user_request);
    println!("{user_try:?}");
}
