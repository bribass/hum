use config::Config;
use std::{env, process};

mod config;

fn raise_error(msg: &str, err: String) -> ! {
    eprintln!("Error: {} ({})", msg, err);
    process::exit(1);
}

fn main() {
    // Find the config file
    let config_filename = env::var("HUM_CONFIG").unwrap_or_else(|err| {
        raise_error(
            "HUM_CONFIG environment variable not defined",
            err.to_string(),
        )
    });

    // Read in the config file
    let config = Config::from_file(config_filename)
        .unwrap_or_else(|err| raise_error("Could not load config file", err.to_string()));
    println!("{:#?}", config);

    // List verbs
    for verb in config.verbs() {
        println!("{}", verb);
    }

    process::exit(0);
}
