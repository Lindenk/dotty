use module::ModuleData;
use error;

pub struct UpdateOptions {
    pub module_name : Option<String>
}

/// Updates links and regenerates files for an installed module
pub fn update(opts : &UpdateOptions) -> Result<(), error::DottyError> {
    /*
    for link in m.links.unwrap() {
        
    }*/
    Ok(())
}