use std::io::{self, Write};

const CALCULATOR_OPERATIONS: &[(&str, fn(f64, f64) -> f64)] = &[
    ("Add", add),
    ("Subtract", subtract),
    ("Multiply", multiply),
    ("Divide", divide),
];

const EXIT_OPTION_INDEX: usize = CALCULATOR_OPERATIONS.len() + 1;

fn display_welcome_message() {
    println!("👋 Welcome to the Rust Calculator 📟\n");
}

fn display_operation_menu() {
    for (index, (name, _)) in CALCULATOR_OPERATIONS.iter().enumerate() {
        println!("{} - {} 🔢", index + 1, name);
    }
    println!("{} - Exit ❌", EXIT_OPTION_INDEX);
}

fn add(num1: f64, num2: f64) -> f64 {
    num1 + num2
}

fn subtract(num1: f64, num2: f64) -> f64 {
    num1 - num2
}

fn multiply(num1: f64, num2: f64) -> f64 {
    num1 * num2
}

fn divide(num1: f64, num2: f64) -> f64 {
    num1 / num2
}

fn read_number(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Failed to read line");

        match num.trim().parse() {
            Ok(n) => return n,
            Err(_) => {
                println!("⚠️ Invalid input. Please enter a valid number.");
            }
        }
    }
}

fn read_first_number() -> f64 {
    read_number("🔢 Enter the first number: ")
}

fn read_second_number() -> f64 {
    read_number("🔢 Enter the second number: ")
}

fn print_result(result: f64) {
    println!("✅ The result is: {}", result);
}

pub fn start() {
    display_welcome_message();

    loop {
        display_operation_menu();

        print!("📝 Enter your choice: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: usize = match choice.trim().parse() {
            Ok(num) if (1..=EXIT_OPTION_INDEX).contains(&num) => num,
            Ok(_) => {
                println!("⚠️ Invalid input. Please enter a number from 1 to {}.", EXIT_OPTION_INDEX);
                continue;
            }
            Err(_) => {
                println!("⚠️ Invalid input. Please enter a number from 1 to {}.", EXIT_OPTION_INDEX);
                continue;
            }
        };

        if choice == EXIT_OPTION_INDEX {
            println!("👋 Exiting the calculator. Goodbye!");
            break;
        }

        let (operation_name, operation) = CALCULATOR_OPERATIONS[choice - 1];

        let num1 = read_first_number();
        let num2 = read_second_number();

        let result = operation(num1, num2);
        println!("🔢 You chose to {}", operation_name);
        print_result(result);
    }
}