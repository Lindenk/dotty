use module::ModuleData;
use error;

pub struct ReinstallOptions {
    pub module_name : Option<String>
}

/// Updates links and regenerates files for an installed module
pub fn reinstall(opts : &ReinstallOptions) -> Result<(), error::DottyError> {
    /*
    for link in m.links.unwrap() {
        
    }*/
    Ok(())
}