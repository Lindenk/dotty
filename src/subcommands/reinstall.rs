use module::Module;
use error;
use config::Config;

pub struct ReinstallOptions {
    pub module_name : Option<String>
}

/// Updates links and regenerates files for an installed module
pub fn reinstall(opts : &ReinstallOptions, conf : Config) -> Result<(), error::DottyError> {
    /*
    for link in m.links.unwrap() {
        
    }*/
    Ok(())
}