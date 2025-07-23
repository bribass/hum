use std::env::Args;
use crate::Action;
use crate::config::{Config, ConfigSection};

pub struct Http;
impl Action for Http {
    fn do_action(&self, config: &Config, action_config: &ConfigSection, args: &mut Args) {
        println!("Called 'http'");
        println!("LDAP Connection: {{{}}}", config.ldap_connection().unwrap());
        println!("Action Config: {{{}}}", action_config.to_string());
        for a in args {
            println!("  arg: '{a}'");
        }
    }
}
