use serde_yaml;
use std::io;

use std::fmt;
use std::error;

#[derive(Debug)]
pub enum DottyError {
    IOError(io::Error),
    YamlError(serde_yaml::Error)
}

impl From<io::Error> for DottyError {
    fn from(e : io::Error) -> DottyError {
        DottyError::IOError(e)
    }
}

impl From<serde_yaml::Error> for DottyError {
    fn from(e : serde_yaml::Error) -> DottyError {
        DottyError::YamlError(e)
    }
}

impl fmt::Display for DottyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("Calling fmt on error.");
        match *self {
            DottyError::IOError(ref e) => e.fmt(f),
            DottyError::YamlError(ref e) => e.fmt(f)
        }
    }
}

impl error::Error for DottyError {
    fn description(&self) -> &str {
        println!("Calling description on error");
        match *self {
            DottyError::IOError(ref e) => e.description(),
            DottyError::YamlError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DottyError::IOError(ref e) => Some(e),
            DottyError::YamlError(ref e) => Some(e),
        }
    }
}