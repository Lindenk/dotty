use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use serde_yaml;

use error::DottyError;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleData {
    links   : Vec<String>,
    append  : HashMap<String, String>,
    hooks   : HashMap<String, String>,
    dependancies : Vec<String>
}

impl ModuleData {
    pub fn load<P: AsRef<Path>>(mod_name : P) -> Result<Self, DottyError> {
        let f : File = try!(File::open(mod_name.as_ref().join("module.yml")));
        let m : ModuleData = try!(serde_yaml::from_reader::<File, Self>(f));
        Ok(m)
    }
}