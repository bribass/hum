use config::Config;
use std::{env, process};

mod config;

fn main() {
    // Find the config file
    let config_filename = env::var("HUM_CONFIG").unwrap_or_else(|err| {
        eprintln!("Error: HUM_CONFIG environment variable not defined ({err})");
        process::exit(1);
    });

    // Read in the config file
    let config = Config::from_file(config_filename).unwrap_or_else(|err| {
        eprintln!("Error: Could not load config file ({err})");
        process::exit(1);
    });
    println!("{config:#?}");

    // List verbs
    for verb in config.verbs() {
        println!("{verb}");
    }

    process::exit(0);
}
