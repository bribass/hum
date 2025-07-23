use std::{env, process, collections::HashMap};
use crate::config::{Config, ConfigSection};

mod config;
mod http;
mod ldap;

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
    let action_config = match config.action(action_name.as_str()) {
        Some(a) => a,
        None => {
            eprintln!("Error: Action '{action_name}' is not defined in the config file.");
            process::exit(1);
        }
    };

    // Determine the mode for the action
    let action_mode = match action_config.get("mode") {
        Some(m) => m.as_str().unwrap_or_else(|| {
            eprintln!("Error: Action '{action_name}' does not define a 'mode' as a string.");
            process::exit(1);
        }),
        None => {
            eprintln!("Error: Action '{action_name}' does not define a 'mode' in the config file.");
            process::exit(1);
        }
    };

    // Dispatch to the correct mode
    let modes = create_modes();
    match modes.get(action_mode) {
        Some(action) => {
            action.do_action(&config, &action_config, &mut args);
        }
        None => {
            eprintln!("Error: Mode '{action_mode}' defined in action '{action_name}' does not exist in the binary.");
            process::exit(1);
        }
    };

    process::exit(0);
}

fn create_modes() -> HashMap<String, Box<dyn Action>> {
    let mut modes = HashMap::<String, Box<dyn Action>>::new();

    modes.insert("query".to_string(), Box::new(ldap::Query));
    modes.insert("modify".to_string(), Box::new(ldap::Modify));
    modes.insert("http".to_string(), Box::new(http::Http));

    modes
}

trait Action {
    fn do_action(&self, config: &Config, action_config: &ConfigSection, args: &mut env::Args);
}
