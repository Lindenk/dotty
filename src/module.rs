use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use file::open;
use serde_yaml;

use error::DottyError;

#[derive(Debug, Serialize, Deserialize)]
struct ModuleData {
    pub name    : String,
    pub links   : Option<Vec<String>>,
    pub append  : Option<HashMap<String, String>>,
    pub hooks   : Option<HashMap<String, String>>,
    pub dependancies : Option<Vec<String>>
}

pub struct Module {
    pub name    : String,
    pub links   : Vec<String>,
    pub append  : HashMap<String, String>,
    pub hooks   : HashMap<String, String>,
    pub dependancies : Vec<String>
}

impl Module {
    pub fn load<P: AsRef<Path>>(mod_name : P) -> Result<Self, DottyError> {
        let f : File = try!(open(mod_name.as_ref().join("module.yml")));
        let m : ModuleData = try!(serde_yaml::from_reader::<File, ModuleData>(f));
        
        Ok(Module{
            name:   m.name,
            links:  m.links.unwrap_or(Vec::new()),
            append: m.append.unwrap_or(HashMap::new()),
            hooks:  m.hooks.unwrap_or(HashMap::new()),
            dependancies:   m.dependancies.unwrap_or(Vec::new())
        })
    }
}