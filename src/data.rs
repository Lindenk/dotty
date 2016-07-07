/// This module allows the storage of system state data 
use error::DottyError;
use config::Config;
use file::{open, create, write, create_dir_all, remove_file};
use utils::to_absolute;

use std::path::PathBuf;
use std::fs::File;

use serde_yaml;

#[derive(Debug, Serialize, Deserialize)]
pub struct InstalledModuleData {
    pub name : String,
    /// A vec of symlinks currently installed by the module
    pub symlinks : Vec<PathBuf>
}

pub fn store_module_data(config : &Config, module_data : &InstalledModuleData) -> 
                                        Result<(), DottyError> {
    try!(create_dir_all(&config.local_data_dir));
    
    // Make sure data is formatted correctly
    let module_data = InstalledModuleData{
        name: module_data.name.clone(),
        symlinks: try!(module_data.symlinks
                        .iter().map(to_absolute).collect())
    };
    
    // Save it
    let serialized_data = serde_yaml::to_string(&module_data).unwrap();
    let mut f = try!(create(config.local_data_dir.join(&module_data.name)));

    try!(write(&mut f, serialized_data.as_bytes()));

    Ok(())
}

pub fn load_module_data(config : &Config, module_name : &str) -> 
                                        Result<InstalledModuleData, DottyError> {
    let f = try!(open(config.local_data_dir.join(&module_name)));
    let result = try!(serde_yaml::from_reader::<File, InstalledModuleData>(f));
    Ok(result)
}

pub fn remove_module(config : &Config, module_name : &str) -> Result<(), DottyError> {
    let path = config.local_data_dir.join(&module_name);
    
    remove_file(&path).map_err(|e| {
        DottyError::IOError(e)
    })
}

pub fn is_module_installed(config : &Config, module_name : &str) -> bool {
    config.local_data_dir.join(module_name).exists()
}