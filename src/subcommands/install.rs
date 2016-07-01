use module::Module;
use error::DottyError;
use config::Config;
use data::{store_module, is_module_installed};

use std::os::unix::fs::symlink;
use std::path::PathBuf;

pub struct InstallOptions {
    pub module_name : String
}

/// Installs a module by running it's hooks, symlinking
/// and generating files as needed
pub fn install(opts : &InstallOptions, conf : &Config) -> Result<(), DottyError> {
    if is_module_installed(&conf, opts.module_name.as_str()) {
        return Err(DottyError::ModuleAlreadyInstalled(opts.module_name.clone()))
    }

    println!("Installing module '{}'...", &opts.module_name);
    let m = try!(Module::load(&opts.module_name));
    
    // Validate module options and config so we don't have a half-broken install
    for link in &m.links {
        let (source, dest) = (&link.0, &link.1);

        println!("Linking '{}' to '{}'...", source.display(), dest.display());
        
        if !source.exists() {
            return Err(DottyError::ModuleMissingFile(source.clone()));
        }
        if dest.exists() {
            return Err(DottyError::ModuleFileAlreadyExists(dest.clone()));
        }
    }

    // Install the module 
    let installed_links : Vec<PathBuf> = vec![];
    for link in m.links {
        match symlink(&link.0, &link.1) {
            Ok(..) => installed_links.push(link),
            Err(e) => println!("Unable to symlink '{}': {}", link, e)
        }
    }

    try!(store_module(&conf, &m));
    
    Ok(())
}