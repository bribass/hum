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

    // Process command line args
    let mut args = env::args();
    args.next();    // Consume the executable filename
    let action_name = match args.next() {
        Some(v) => v,
        None => {
            eprintln!("Error: No action specified.");
            process::exit(1);
        }
    };
    let action = match config.action(action_name.as_str()) {
        Some(a) => a,
        None => {
            eprintln!("Error: Action '{action_name}' is not defined in the config file.");
            process::exit(1);
        }
    };

    // Determine the mode for the action
    let action_mode = match action.get("mode") {
        Some(m) => m.as_str().unwrap_or_else(|| {
            eprintln!("Error: Action '{action_name}' does not define a 'mode' as a string.");
            process::exit(1);
        }),
        None => {
            eprintln!("Error: Action '{action_name}' does not define a 'mode' in the config file.");
            process::exit(1);
        }
    };
    println!("{action_name} => {action_mode}");

    process::exit(0);
}
