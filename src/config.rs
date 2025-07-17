use std::error::Error;
use std::fmt::Debug;
use std::fs;
use toml::Table;

/// Encapsulate all information from a configuration file.
#[derive(Debug)]
pub struct Config {
    config: Table,
}

pub type ConfigSection = Table;

impl Config {
    /// Read a configuration file into the program.
    ///
    /// # Arguments
    ///
    /// * `filename`: The filename of the configuration file.
    ///
    /// returns: `Result<Config, Box<dyn Error, Global>>`
    pub fn from_file(filename: String) -> Result<Config, Box<dyn Error>> {
        // Read in the config file
        let contents = fs::read_to_string(filename)?;
        let table = contents.parse::<Table>()?;
        Ok(Config {
            config: table,
        })
    }

    /// Returns a section of the configuration file as defined by a TOML table.
    ///
    /// # Arguments
    ///
    /// * `name`: The name of the section to return.
    ///
    /// returns: `Option<&Map<String, Value>>`
    pub fn get_section(&self, name: &str) -> Option<&ConfigSection> {
        self.config.get(name).unwrap().as_table()
    }

    /// Returns an `Iterator` that lists all verbs defined in the configuration file.
    ///
    /// returns: `impl Iterator<Item = &String>`
    pub fn verbs(&self) -> impl Iterator<Item = &String> {
        let verb_table = self.get_section("verb").unwrap();
        verb_table.keys()
    }
}
