use module::ModuleData;
use error;

pub struct InstallOptions {
    
}

/// Installs a module by running it's hooks, symlinking
/// and generating files as needed
pub fn install(opts : &InstallOptions) -> Result<(), error::DottyError> {
    /*
    for link in m.links.unwrap() {
        
    }*/
    Ok(())
}