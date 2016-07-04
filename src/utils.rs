/// This file contain misc functions that are not built into the language, 
/// or the standard library, but are still needed for this program

use std::path::PathBuf;
use std::env::{home_dir, current_dir};

use error::DottyError;
use file::FileError;

/*
pub fn path_from_env(env_name : &str) -> Result<PathBuf, DottyError> {
    if let Some(env) = var_os(&env_name) {
        let paths = split_paths(&env).collect::<Vec<_>>();
        Ok(PathBuf::from(join_paths(paths).unwrap()))
    }
    else {
        Err(DottyError::EnvNotFound(env_name.to_string()))
    }
}*/

pub fn to_absolute(path : &PathBuf) -> Result<PathBuf, FileError> {
    if path.is_relative() {
        let mut c_dir = try!(current_dir().map_err(|e| FileError(e, PathBuf::from("."))));
        c_dir.push(path);
        Ok(c_dir)
    } else {
        Ok(path.clone())
    }
}

pub fn resolve_tilde(path : &PathBuf) -> Result<PathBuf, DottyError> {
    let home = try!(home_dir().ok_or(
        DottyError::EnvNotFound(String::from("HOME"))
    ));

    if let Ok(path) = path.strip_prefix("~") {
        Ok(home.join(path))
    } else {
        Ok(path.clone())
    }
}

#[test]
fn test_resolve_tilde_simple() {
    let correct_path = home_dir();
    let result = resolve_tilde(&PathBuf::from("~")).ok();

    assert_eq!(correct_path, result);
}

#[test]
fn test_resolve_tilde_complex() {
    let correct_path = home_dir();
    let result = resolve_tilde(&PathBuf::from("~/a/longer/path")).ok();

    if let Some(mut p) = correct_path {
        p.push("a/longer/path");
        assert_eq!(Some(p), result);
    } else {
        assert_eq!(None, result)
    }
}