use serde_yaml;
use file::FileError;

use std::fmt;
use std::error;

#[derive(Debug)]
pub enum DottyError {
    IOError(FileError),
    YamlError(serde_yaml::Error),
    ConfigError(String),
    EnvNotFound(String)
}

impl From<serde_yaml::Error> for DottyError {
    fn from(e : serde_yaml::Error) -> DottyError {
        DottyError::YamlError(e)
    }
}

impl From<FileError> for DottyError {
    fn from(e : FileError) -> DottyError {
        DottyError::IOError(e)
    }
}

impl fmt::Display for DottyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("Calling fmt on error.");
        match *self {
            DottyError::IOError(ref e) => e.fmt(f),
            DottyError::YamlError(ref e) => e.fmt(f),
            DottyError::ConfigError(ref msg) => write!(f, "{}", msg),
            DottyError::EnvNotFound(ref env) => write!(f, "Can't find environment variable: '${}'", env)
        }
    }
}

impl error::Error for DottyError {
    fn description(&self) -> &str {
        println!("Calling description on error");
        match *self {
            DottyError::IOError(ref e) => e.description(),
            DottyError::YamlError(ref e) => e.description(),
            DottyError::ConfigError(ref msg) => msg,
            DottyError::EnvNotFound(..) => "Can't find environment variable."
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DottyError::IOError(ref e) => Some(e),
            DottyError::YamlError(ref e) => Some(e),
            DottyError::ConfigError(..) => None,
            DottyError::EnvNotFound(..) => None
        }
    }
}