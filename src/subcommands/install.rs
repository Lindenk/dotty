use module::Module;
use error::DottyError;
use config::Config;
use data::{InstalledModuleData, store_module_data, is_module_installed};
use utils::recursive_symlink;

use std::path::PathBuf;

pub struct InstallOptions {
    pub module_name : String,
    pub module_path : PathBuf
}

/// Installs a module by running it's hooks, symlinking
/// and generating files as needed
pub fn install(opts : &InstallOptions, conf : &Config) -> Result<(), DottyError> {
    if is_module_installed(&conf, opts.module_name.as_str()) {
        return Err(DottyError::ModuleAlreadyInstalled(opts.module_name.clone()))
    }

    println!("Installing module '{}'...", &opts.module_name);
    let m = try!(Module::load(&opts.module_path));
    
    // Validate module options and config so we don't have a half-broken install
    for link in &m.links {
        let (source, dest) = (&link.0, &link.1);

        println!("Linking '{}' to '{}'...", source.display(), dest.display());
        
        if !source.exists() {
            return Err(DottyError::ModuleMissingFile(source.clone()));
        }
        /*if dest.exists() {
            return Err(DottyError::ModuleFileAlreadyExists(dest.clone()));
        }*/
    }

    // Install the module 
    let mut installed_links : Vec<PathBuf> = vec![];
    for link in m.links {
        let (source, dest) = (link.0, link.1);
        match recursive_symlink(&source, &dest) {
            Ok(v) => installed_links.extend_from_slice(&v),
            Err(e) => println!("Unable to symlink '{}' to '{}' : {}", 
                                    source.display(), dest.display(), e)
        }
    }

    try!(store_module_data(&conf, &InstalledModuleData{
        name: opts.module_name.clone(),
        symlinks: installed_links,
    }));
    
    Ok(())
}