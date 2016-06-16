use std::path::PathBuf;
use std::env::{var_os, split_paths, join_paths};

use error::DottyError;
use consts;

use std::path::Path;
use std::fs::File;
use file::open;
use serde_yaml;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigBuilder {
    pub local_data_dir : Option<PathBuf>,
}

pub struct Config {
    pub local_data_dir : PathBuf
}

impl ConfigBuilder {
    pub fn default() -> ConfigBuilder {
        let local_data_dir = if let Some(home_dir) = var_os("HOME") {
            let mut paths = split_paths(&home_dir).collect::<Vec<_>>();
            paths.push(PathBuf::from(consts::DEFAULT_DATA_DIR));
            Some(PathBuf::from(join_paths(paths).unwrap()))
        } else {
            None
        };
        
        ConfigBuilder { 
            local_data_dir: local_data_dir
        }
    }
    
    pub fn load<P: AsRef<Path>>(path : P) -> Result<ConfigBuilder, DottyError> {
        let f : File = try!(open(path.as_ref()));
        let loaded_config : ConfigBuilder = try!(serde_yaml::from_reader::<File, Self>(f));
        Ok(loaded_config)
    }
    
    pub fn load_into<P: AsRef<Path>>(&self, path : P) -> Result<ConfigBuilder, DottyError> {
        let loaded_config = try!(ConfigBuilder::load(path));
        
        Ok(ConfigBuilder {
            local_data_dir: loaded_config.local_data_dir.or(self.local_data_dir.clone())
        })
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