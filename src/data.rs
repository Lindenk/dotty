/// This module allows the storage of system state data 
use error::DottyError;
use config::Config;
use module::Module;
use file::{create, create_dir_all, remove_file};

use std::io::Write;

pub fn store_module(config : &Config, module : &Module) -> Result<(), DottyError> {
    try!(create_dir_all(&config.local_data_dir.join(&module.name)));
    let mut f = try!(create(config.local_data_dir.join(&module.name).join("module.yml")));

    f.write(module.serialize().as_bytes()).unwrap();

    Ok(())
}

pub fn load_module(config : &Config, module_name : &str) -> Result<Module, DottyError> {
    Module::load(config.local_data_dir.join(&module_name))
}

pub fn remove_module(config : &Config, module_name : &str) -> Result<(), DottyError> {
    let path = config.local_data_dir.join(&module_name).join("module.yml");
    
    remove_file(&path).map_err(|e| {
        DottyError::IOError(e)
    })
}

pub fn is_module_installed(config : &Config, module_name : &str) -> bool {
    config.local_data_dir.join(module_name).join("module.yml").exists()
}