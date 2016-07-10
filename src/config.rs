use std::path::PathBuf;

use error::DottyError;
use consts;

use std::path::Path;
use std::fs::File;
use file::open;
use serde_yaml;

use utils::resolve_tilde;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigBuilder {
    pub local_data_dir : Option<PathBuf>,
}

pub struct Config {
    pub local_data_dir : PathBuf
}

impl ConfigBuilder {
    pub fn default() -> ConfigBuilder {
        let local_data_dir = resolve_tilde(
            &PathBuf::from(consts::DEFAULT_DATA_DIR)).ok();
        
        ConfigBuilder { 
            local_data_dir: local_data_dir
        }
    }
    
    pub fn load<P: AsRef<Path>>(path : P) -> Result<ConfigBuilder, DottyError> {
        let f : File = try!(open(path.as_ref()));
        let mut loaded_config : ConfigBuilder = try!(serde_yaml::from_reader::<File, Self>(f));

        if let Some(d) = loaded_config.local_data_dir {
            loaded_config.local_data_dir = Some(try!(resolve_tilde(&d)));
        }

        Ok(loaded_config)
    }
    
    pub fn load_into<P: AsRef<Path>>(self, path : P) -> ConfigBuilder {
        let loaded_config = ConfigBuilder::load(path);
        
        match loaded_config {
            Ok(c) => ConfigBuilder {
                local_data_dir: c.local_data_dir.or(self.local_data_dir.clone())
            },
            Err(..) => self
        }
    }
    
    pub fn validate(&self) -> Result<Config, DottyError> {
        let local_data_dir = match self.local_data_dir {
            Some(ref x) => x,
            None => return Err(DottyError::ConfigError("Data directory not specified and $HOME not set.".to_string()))
        };
        
        Ok(Config {
            local_data_dir: local_data_dir.clone()
        })
    }
}