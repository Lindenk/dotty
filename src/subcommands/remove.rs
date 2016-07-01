use module::Module;
use error::DottyError;
use config::Config;
use data::{load_module, remove_module, is_module_installed};
use file::remove_file;

use std::env::current_dir;

pub struct RemoveOptions {
    pub module_name : String
}

/// Removes an installed module, removing any installed files
/// associated with it, and calling any remove hooks.
pub fn remove(opts : &RemoveOptions, conf : &Config) -> Result<(), DottyError> {
    if !is_module_installed(&conf, opts.module_name.as_str()) {
        return Err(DottyError::ModuleNotInstalled(opts.module_name.clone()))
    }

    println!("Removing module '{}'...", &opts.module_name);
    let m = try!(load_module(&conf, opts.module_name.as_str()));
    
    // Validate module options and config so we don't have a half-broken install
    for link in &m.links {
        let dest = &link.1;

        println!("Removing link '{}'", dest.display());
        
        try!(remove_file(dest));
    }

    try!(remove_module(&conf, opts.module_name.as_str()));
    
    Ok(())
}