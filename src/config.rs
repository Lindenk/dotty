use std::path::PathBuf;
use std::env::{var_os, split_paths, join_paths};

pub struct Config {
    pub local_data_dir : PathBuf,
}

impl Config {
    pub fn new() -> Config {
        if let Some(home_dir) = var_os("HOME") {
            let mut paths = split_paths(&home_dir).collect::<Vec<_>>();
            paths.push(PathBuf::from(consts::DEFAULT_DATA_DIR));
            Config{ local_data_dir: PathBuf::from(join_paths(paths).unwrap()) }
        } else {
            Config{ local_data_dir: PathBuf::from("/tmp/dotty") }
        }
    }
}