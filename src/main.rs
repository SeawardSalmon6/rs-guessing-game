use chrono::{DateTime, Utc};
use rand::Rng;
use std::{
    cmp::Ordering,
    io::{self, Write},
};

enum FinishGameStatus {
    Win,
    Lose,
}

struct FinishGameMessage {
    status: FinishGameStatus,
    secret_number: Option<u8>,
    start_time: Option<DateTime<Utc>>,
}

const ONE_SECOND_IN_MILLISECONDS: i64 = 1_000;

fn main() {
    println!("Guess the number!");

    let start_time = Utc::now();
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Please input your guess (or 'quit' to quit the game): ");
        io::stdout().flush().expect("Flush failed");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        if guess.to_lowercase().eq_ignore_ascii_case("quit") {
            finish_game(FinishGameMessage {
                status: FinishGameStatus::Lose,
                start_time: Some(start_time),
                secret_number: Some(secret_number),
            });
            break;
        }

        let guess = match guess.parse::<i32>() {
            Result::Ok(n) => n,
            Result::Err(_) => {
                println!("Try again. Type a number or 'quit'.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&(secret_number as i32)) {
            Ordering::Less => println!("It's bigger! ↗️"),
            Ordering::Greater => println!("It's lesser! ↘️"),
            Ordering::Equal => {
                finish_game(FinishGameMessage {
                    status: FinishGameStatus::Win,
                    start_time: Some(start_time),
                    secret_number: Some(secret_number),
                });
                break;
            }
        }
    }
}

fn finish_game(action: FinishGameMessage) {
    match action.status {
        FinishGameStatus::Win => {
            let end_time = Utc::now();
            let diff = end_time - action.start_time.unwrap();
            let milliseconds = diff.num_milliseconds();

            let seconds = milliseconds / ONE_SECOND_IN_MILLISECONDS;
            let milliseconds = milliseconds - (seconds * ONE_SECOND_IN_MILLISECONDS);

            println!(
                "You guessed it! You took {}.{} seconds",
                seconds, milliseconds
            );
        }
        FinishGameStatus::Lose => {
            println!(
                "Haha, you didn't guess it! It was {}.",
                action.secret_number.unwrap()
            );
        }
    }
}
