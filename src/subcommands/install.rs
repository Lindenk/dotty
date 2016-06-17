use module::Module;
use error;
use config::Config;

use std::os::unix::fs::symlink;
use std::path::PathBuf;

pub struct InstallOptions {
    pub module_name : String
}

/// Installs a module by running it's hooks, symlinking
/// and generating files as needed
pub fn install(opts : &InstallOptions, conf : Config) -> Result<(), error::DottyError> {
    let m = try!(Module::load(&opts.module_name));
    
    for link in m.links {
        let split_link : Vec<&str> = link.split(':').collect();
        if split_link.len() != 2 {
            return Err(error::DottyError::ModuleSyntaxError(link.clone()))
        }
        
        let (source, dest) = (PathBuf::from(split_link[0]),
                              PathBuf::from(split_link[1]));
        if !source.exists() {
            return Err(error::DottyError::ModuleError(source, "Source file does not exist")
        }
    }
    
    Ok(())
}