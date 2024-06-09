use clap::{Arg, Command};

pub fn run() {
    let matches = Command::new("XSO LEARNING RUST CLI")
        .version("0.1")
        .author("Abdulrahman Nader <abdulrahmanxso25@outlook.com>")
        .about("🚀 A CLI tool to learn Rust by running various project modules")
        .arg(Arg::new("calculator")
            .long("calculator")
            .help("🧮 Runs the calculator module: a basic calculator with arithmetic operations"))
        .arg(Arg::new("guessing_game")
            .long("guessing_game")
            .help("🎮 Runs the guessing_game module: a simple guessing game"))
        .get_matches();

    if matches.is_present("calculator") {
        println!("🧮 Starting the calculator module...");
        super::calculator::start();
    } else if matches.is_present("guessing_game") {
        println!("🎮 Starting the guessing game module...");
        super::guessing_game::start();
    } else {
        println!("⚠️ Please provide a valid argument: --calculator or --guessing_game");
        println!("🛠️  Use --help for more information about available commands.");
    }
}
