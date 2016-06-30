use module::Module;
use error;
use config::Config;

pub struct UpdateOptions {
    pub module_name : Option<String>
}

/// Updates links and regenerates files for an installed module
pub fn update(opts : &UpdateOptions, conf : &Config) -> Result<(), error::DottyError> {
    /*
    for link in m.links.unwrap() {
        
    }*/
    Ok(())
}