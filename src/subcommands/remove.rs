use module::ModuleData;
use error;

pub struct RemoveOptions {
    pub module_name : String
}

/// Removed an installed module, removing any installed files
/// associated with it, and calling any remove hooks.
pub fn remove(opts : &RemoveOptions) -> Result<(), error::DottyError> {
    /*
    for link in m.links.unwrap() {
        
    }*/
    Ok(())
}