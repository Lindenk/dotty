use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::File;
use file::open;
use serde_yaml;

use error::DottyError;
use utils::resolve_tilde;

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
    pub links   : Vec<(PathBuf, PathBuf)>,
    pub append  : HashMap<String, String>,
    pub hooks   : HashMap<String, String>,
    pub dependancies : Vec<String>
}

impl Module {
    pub fn load<P: AsRef<Path>>(mod_path : P) -> Result<Self, DottyError> {
        let f : File = try!(open(mod_path.as_ref().join("module.yml")));
        let m : ModuleData = try!(serde_yaml::from_reader::<File, ModuleData>(f));

        // Validate data and parse options
        let links : Vec<String> = m.links.unwrap_or(Vec::new());
        let parsed_links : Vec<(PathBuf, PathBuf)> = 
                                        try!(links.into_iter().map(|link| {
            let split_link : Vec<&str> = link.split(':').collect();
            if split_link.len() != 2 {
                return Err(DottyError::ModuleSyntaxError(link.clone()))
            };
            
            let (source, dest) = (mod_path.as_ref().join(PathBuf::from(split_link[0])),
                                try!(resolve_tilde(&PathBuf::from(split_link[1]))));

            Ok((source, dest))
        }).collect());

        Ok(Module{
            name:   m.name,
            links:  parsed_links,
            append: m.append.unwrap_or(HashMap::new()),
            hooks:  m.hooks.unwrap_or(HashMap::new()),
            dependancies:   m.dependancies.unwrap_or(Vec::new())
        })
    }

    pub fn serialize(&self) -> String {
        let links : Vec<String> = (&self.links).into_iter().map(|link| {
            let (source, dest) = (&link.0, &link.1);
            format!("{}:{}", source.display(), dest.display())
        }).collect();

        let m_data = ModuleData {
            name: self.name.clone(),
            links: Some(links),
            append: Some(self.append.clone()),
            hooks: Some(self.hooks.clone()),
            dependancies: Some(self.dependancies.clone()),
        };

        serde_yaml::to_string(&m_data).unwrap()
    }
}