use std::io::{self, Write};
use rand::Rng;

const MIN_RANGE: i32 = 1;
const MAX_RANGE: i32 = 9;
const EXIT_OPTION: &str = "exit";

fn display_welcome_message() {
    println!("👋 Welcome to the Rust Guessing Game 🎲\n");
}

fn display_instructions() {
    println!("Guess a number between {} and {}.", MIN_RANGE, MAX_RANGE);
    println!("Type '{}' to exit the game ❌", EXIT_OPTION);
}

fn read_guess() -> Option<i32> {
    loop {
        print!("Enter your guess or type '{}' to exit: ", EXIT_OPTION);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim().eq_ignore_ascii_case(EXIT_OPTION) {
            return None;
        }

        match input.trim().parse::<i32>() {
            Ok(num) => return Some(num),
            Err(_) => println!("⚠️ Invalid input. Please enter a valid number or type '{}' to exit.", EXIT_OPTION),
        }
    }
}

fn check_guess(guess: i32, secret_number: i32) -> bool {
    if guess == secret_number {
        println!("🎉 Correct! You guessed the number!");
        true
    } else {
        println!("❌ Incorrect. Try again!");
        false
    }
}

pub fn start() {
    display_welcome_message();
    let secret_number = rand::thread_rng().gen_range(MIN_RANGE..=MAX_RANGE);

    display_instructions();

    loop {
        if let Some(guess) = read_guess() {
            if check_guess(guess, secret_number) {
                break;
            }
        } else {
            println!("👋 Exiting the game. Thanks for playing!");
            break;
        }
    }
}