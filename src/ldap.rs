use std::env::Args;
use crate::Action;
use crate::config::{Config, ConfigSection};

pub struct Query;
impl Action for Query {
    fn do_action(&self, config: &Config, action_config: &ConfigSection, args: &mut Args) {
        println!("Called 'query'");
        println!("LDAP Connection: {{{}}}", config.ldap_connection().unwrap());
        println!("Action Config: {{{}}}", action_config.to_string());
        for a in args {
            println!("  arg: '{a}'");
        }
    }
}

pub struct Modify;
impl Action for Modify {
    fn do_action(&self, config: &Config, action_config: &ConfigSection, args: &mut Args) {
        println!("Called 'modify'");
        println!("LDAP Connection: {{{}}}", config.ldap_connection().unwrap());
        println!("Action Config: {{{}}}", action_config.to_string());
        for a in args {
            println!("  arg: '{a}'");
        }
    }
}
