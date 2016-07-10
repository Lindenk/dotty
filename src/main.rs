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
mod data;

use cli::parse_cli_args;
use config::ConfigBuilder;
use utils::resolve_tilde;

use std::path::PathBuf;

fn main() {
    // Parse cli options
    let yml = load_yaml!("cli.yaml");
    let cli_args = clap::App::from_yaml(yml).get_matches();
    
    // Load config
    let config_path = resolve_tilde(&PathBuf::from(cli_args.value_of("config_dir")
                            .unwrap_or(consts::DEFAULT_USER_CONFIG_FILE)))
                        .unwrap_or_else(|e| {
                            println!("Failed to resolve config file path: {}", e);
                            exit(1);});
    let config =    ConfigBuilder::default()
                    .load_into(config_path);
    let config =    config.validate()
                    .unwrap_or_else(|e| {
                        println!("{}", e);
                        exit(1);});
        
    // Execute subcommand
    let command = parse_cli_args(cli_args);
    command.run(&config).unwrap_or_else(|e| {
        println!("{}", e);
        exit(1);
    });
}
