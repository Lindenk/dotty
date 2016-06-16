/// This is a wrapper for std::fs::File that will correctly return 
/// path information in it's errors

use std::io;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::fmt;
use std::error;

#[derive(Debug)]
pub struct FileError(io::Error, PathBuf);

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1.to_string_lossy())
    }
}

impl error::Error for FileError {
    fn description(&self) -> &str {
        format!("{}: {}", self.0, self.1.display());
        ""
    }

    fn cause(&self) -> Option<&error::Error> {
        Some(&self.0)
    }
}

pub fn open<P : AsRef<Path>>(path : P) -> Result<File, FileError> {
    let cloned_path = path.as_ref().clone();
    File::open(cloned_path).map_err(|e| FileError(e, cloned_path.to_path_buf()))
}