#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate clap;
extern crate serde;
extern crate serde_yaml;

use std::process::exit;

mod consts;
mod cli;
mod module;
mod error;
mod subcommands;
mod config;
mod utils;
mod file;

use cli::parse_cli_args;
use config::ConfigBuilder;
use utils::path_from_env;

fn main() {
    // Parse cli options
    let yml = load_yaml!("src/cli.yaml");
    let cli_args = clap::App::from_yaml(yml).get_matches();
    
    // Load config
    let mut config_path = path_from_env("HOME")
                          .unwrap_or_else(|e| {
                              println!("{}", e); exit(1);});
    config_path.push(cli_args.value_of("config_dir")
               .unwrap_or(consts::DEFAULT_USER_CONFIG_FILE));
    let config =    ConfigBuilder::default()
                    .load_into(config_path)
                    .unwrap_or_else(|e| {
                        println!("{}", e);
                        exit(1);});
    let config =    config.validate()
                    .unwrap_or_else(|e| {
                        println!("{}", e);
                        exit(1);});
        
    // Execute subcommand
    let command = parse_cli_args(cli_args);
    command.run(config).unwrap_or_else(|e| {
        println!("{}", e);
        exit(1);
    });
}
