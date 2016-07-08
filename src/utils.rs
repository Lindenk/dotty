/// This file contain misc functions that are not built into the language, 
/// or the standard library, but are still needed for this program

use std::path::{Path, PathBuf};
use std::env::{home_dir, current_dir};

use error::DottyError;
use file::FileError;
use std::io;

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


/// This function will symlink a file or directory and, if it is a directory and already exists, it will 
/// attempt to symlink the files/directories in the source directory instead. File conflicts will return an 
/// error. Returns a vec of paths to all symlinks created.
pub fn recursive_symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<Vec<PathBuf>, FileError> {
    let mut symlinked_files : Vec<PathBuf> = vec![];
    
    let dst = dst.as_ref();
    let src = try!(to_absolute(&PathBuf::from(src.as_ref())));
    
    if src.is_dir() && dst.exists() {
                        
        if !dst.is_dir() {
            return Err(FileError(io::Error::new(io::ErrorKind::AlreadyExists, "Attempted to symlink a directory to an existing file."), PathBuf::from(dst)));
        }
        
        // We should be all clear to recurse here
        let dst_iter = try!(src.read_dir().map_err(|e| FileError(e, PathBuf::from(src))));
        for f in dst_iter {
            let f = try!(f.map_err(|e| FileError(e, PathBuf::from(dst))));
            let mut new_dst = PathBuf::from(dst);
            new_dst.push(f.file_name());
            let r_links = try!(recursive_symlink(f.path(), &new_dst));
            symlinked_files.extend_from_slice(&r_links);
        }
    } else {
        try!(symlink(&src, dst).map_err(|e| FileError(e, PathBuf::from(src))));
        symlinked_files.push(PathBuf::from(dst));
    }
    
    Ok(symlinked_files)
}


// Platform specific code 
#[cfg(any(target_os = "macos", target_os = "linux"))]
mod os_utils {
    use std::os::unix::fs::symlink as unix_symlink;
    use std::path::Path;
    use std::io;

    pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<(), io::Error> {
        unix_symlink(src, dst)
    }
}

#[cfg(target_os = "windows")]
mod os_utils {
    use std::os::windows::fs::{symlink_file, symlink_dir};
    use std::path::Path;
    use std::io;

    pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<(), io::Error> {
        if src.as_ref().is_file() {
            symlink_file(src, dst)
        } else if src.as_ref().is_dir() {
            symlink_dir(src, dst)
        } else {
            unreachable!();
        }
    }
}

pub use self::os_utils::*;

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