/// This module allows the storage of system state data 
use error::DottyError;
use config::Config;
use module::Module;
use file::{create, create_dir_all};

use std::io::Write;

pub fn store_module(config : &Config, module : &Module) -> Result<(), DottyError> {
    try!(create_dir_all(&config.local_data_dir));
    let mut f = try!(create(config.local_data_dir.join(&module.name)));

    f.write(module.serialize().as_bytes()).unwrap();

    Ok(())
}

pub fn is_module_installed(config : &Config, module_name : &str) -> bool {
    config.local_data_dir.join(module_name).exists()
}