use rand::Rng;
use std::{
    cmp::Ordering,
    io::{self, Write},
};

fn main() {
    println!("Guess the number!");

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
            println!("Haha, you didn't guess it! It was {secret_number}.");
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
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        }
    }
}
