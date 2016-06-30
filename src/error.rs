use serde_yaml;
use file::FileError;
use std::path::PathBuf;

use std::fmt;
use std::error;

#[derive(Debug)]
pub enum DottyError {
    IOError(FileError),
    YamlError(serde_yaml::Error),
    ConfigError(String),
    EnvNotFound(String),
    
    ModuleSyntaxError(String),
    ModuleAlreadyInstalled(String),
    ModuleMissingFile(PathBuf),
    ModuleFileAlreadyExists(PathBuf),
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
        match *self {
            DottyError::IOError(ref e) => e.fmt(f),
            DottyError::YamlError(ref e) => e.fmt(f),
            DottyError::ConfigError(ref msg) => write!(f, "Error in config file: {}", msg),
            DottyError::EnvNotFound(ref env) => write!(f, "Can't find environment variable: '${}'", env),
            DottyError::ModuleSyntaxError(ref s) => write!(f, "Module Syntax Error: {}", s),
            DottyError::ModuleAlreadyInstalled(ref s) => write!(f, "Module Already Installed: {}", s),
            DottyError::ModuleMissingFile(ref p) => write!(f, "Module file or directory is missing: {}", p.display()),
            DottyError::ModuleFileAlreadyExists(ref p) => write!(f, "File already exists: {}", p.display()),
        }
    }
}

impl error::Error for DottyError {
    fn description(&self) -> &str {
        match *self {
            DottyError::IOError(ref e) => e.description(),
            DottyError::YamlError(ref e) => e.description(),
            DottyError::ConfigError(ref msg) => msg,
            DottyError::EnvNotFound(..) => "Can't find environment variable.",
            DottyError::ModuleSyntaxError(..) => "Module syntax error.",
            DottyError::ModuleAlreadyInstalled(..) => "Module already installed.",
            DottyError::ModuleMissingFile(..) => "Module file or directory missing.",
            DottyError::ModuleFileAlreadyExists(..) => "Module file already exists.",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DottyError::IOError(ref e) => Some(e),
            DottyError::YamlError(ref e) => Some(e),
            DottyError::ConfigError(..) => None,
            DottyError::EnvNotFound(..) => None,
            DottyError::ModuleSyntaxError(..) => None,
            DottyError::ModuleAlreadyInstalled(..) => None,
            DottyError::ModuleMissingFile(..) => None,
            DottyError::ModuleFileAlreadyExists(..) => None,
        }
    }
}