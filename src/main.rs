#[macro_use]
extern crate clap;

use std::path::PathBuf;
use std::env::{var_os, split_paths, join_paths};

mod consts;

struct Config {
    pub local_data_dir : PathBuf,
}

impl Config {
    fn new() -> Config {
        if let Some(home_dir) = var_os("HOME") {
            let mut paths = split_paths(&home_dir).collect::<Vec<_>>();
            paths.push(PathBuf::from(consts::DEFAULT_DATA_DIR));
            Config{ local_data_dir: PathBuf::from(join_paths(paths).unwrap()) }
        } else {
            Config{ local_data_dir: PathBuf::from("/tmp/dotty") }
        }
    }
}

fn main() {
    // Parse cli options
    let yml = load_yaml!("cli.yaml");
    let cli_args = clap::App::from_yaml(yml).get_matches();
    
    let _ = Config::new();
    
    let get_module_name = |m| cli_args.subcommand_matches(m).unwrap().value_of("module_name").unwrap();
    
    // only run impure actions once we are certain they will work
    let (command, module) = match cli_args.subcommand_name() {
        Some(m @ "install") | Some(m @ "remove") | Some(m @ "update") | Some(m @ "reinstall") => {
            //proc_install(get_module_name(m))
            (m, get_module_name(m))
        }
        _ => unreachable!()
    };
    
    println!("{} {}", command, module);
}
