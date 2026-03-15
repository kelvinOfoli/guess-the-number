mod animation;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::Colorize;

fn main() {
    animation::startup_sequence();

    // Generate a random number between 1 and 100 (inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut attempts = 0u32;
    let mut first_guess = true;

    loop {
        animation::type_out("\nEnter your guess:", 25);

        // Create a mutable String to hold the user's input
        let mut guess = String::new();

        let bytes_read = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 0 bytes means EOF (end of input) — stop the game
        if bytes_read == 0 {
            println!("No input. Exiting.");
            break;
        }

        // Convert the String to a number; skip invalid input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                animation::animate_invalid_input();
                continue;
            }
        };

        attempts += 1;

        // Compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                animation::animate_too_low();
                let warmth = animation::warmth_percentage(guess, secret_number);
                animation::animate_warmth_bar(warmth, first_guess);
                first_guess = false;
            }
            Ordering::Greater => {
                animation::animate_too_high();
                let warmth = animation::warmth_percentage(guess, secret_number);
                animation::animate_warmth_bar(warmth, first_guess);
                first_guess = false;
            }
            Ordering::Equal => {
                animation::win_celebration(attempts);
                let msg = format!("Correct! You got it in {} attempt(s)!", attempts);
                animation::type_out(&format!("{}", msg.bold().green()), 35);
                break;
            }
        }
    }
}
