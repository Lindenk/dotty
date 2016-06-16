/// This file contain misc functions that are not built into the language, 
/// or the standard library, but are still needed for this program

use std::path::PathBuf;
use std::env::{var_os, split_paths, join_paths};

use error::DottyError;

pub fn path_from_env(env_name : &str) -> Result<PathBuf, DottyError> {
    if let Some(env) = var_os(&env_name) {
        let paths = split_paths(&env).collect::<Vec<_>>();
        Ok(PathBuf::from(join_paths(paths).unwrap()))
    }
    else {
        Err(DottyError::EnvNotFound(env_name.to_string()))
    }
}