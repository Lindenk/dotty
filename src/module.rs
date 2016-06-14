use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use serde_yaml;

use error::DottyError;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleData {
    pub name    : String,
    pub links   : Option<Vec<String>>,
    pub append  : Option<HashMap<String, String>>,
    pub hooks   : Option<HashMap<String, String>>,
    pub dependancies : Option<Vec<String>>
}

impl ModuleData {
    pub fn load<P: AsRef<Path>>(mod_name : P) -> Result<Self, DottyError> {
        let f : File = try!(File::open(mod_name.as_ref().join("module.yml")));
        let m : ModuleData = try!(serde_yaml::from_reader::<File, Self>(f));
        Ok(m)
    }
}