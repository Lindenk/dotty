use module::ModuleData;
use error;
use config::Config;

pub struct InstallOptions {
    pub module_name : String
}

/// Installs a module by running it's hooks, symlinking
/// and generating files as needed
pub fn install(opts : &InstallOptions, conf : Config) -> Result<(), error::DottyError> {
    /*
    for link in m.links.unwrap() {
        
    }*/
    Ok(())
}