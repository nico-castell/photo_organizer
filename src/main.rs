use std::{env, process};

use iphone_organizer::Config;

fn main() {
    // Handle `--help` and `-h`
    if env::args().find(|arg| arg == "--help" || arg == "-h").is_some() {
        Config::print_config();
        process::exit(0);
    }

    // Build config
    let config = match Config::build(env::args()) {
        Ok(args) => args,
        Err(error) => {
            eprintln!("Configuration error: {}", error);
            process::exit(1);
        }
    };

    // Run the program
    if let Err(error) = iphone_organizer::run(config) {
        eprintln!("Application error: {}", error);
        process::exit(2);
    }
}
