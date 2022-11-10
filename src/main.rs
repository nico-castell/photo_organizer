use std::{env, process};

use photo_organizer::Config;

fn main() {
    let config = match Config::build(env::args()) {
        Ok(args) => args,
        Err(error) => {
            eprintln!("Configuration error: {}", error);
            process::exit(1);
        }
    };

    if let Err(error) = photo_organizer::run(config) {
        eprintln!("Application error: {}", error);
        process::exit(2);
    }
}
