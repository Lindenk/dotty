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

use cli::parse_cli_args;

fn main() {    
    // Load config
    //let _ = Config::new();
    
    // Parse cli options
    let yml = load_yaml!("src/cli.yaml");
    let cli_args = clap::App::from_yaml(yml).get_matches();
    let command = parse_cli_args(cli_args);
    
    // Execute subcommand
    //let mod_name = cli_args.subcommand_matches(m).unwrap().value_of("module_name");
    command.run().unwrap_or_else(|e| {
        println!("{}", e);
        exit(1);
    });
    
    //println!("{} {}", command, module);
    /*
    let m = match ModuleData::load(module){
        Ok(m) => m,
        Err(e) => {
            println!("Error when parsing module config '{}':\n{}", module, e);
            exit(1);
        }
    };
    
    println!("{:?}", m);
    */
}
