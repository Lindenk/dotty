
/*
extern crate dotty;

use std::env::home_dir;
use std::path::PathBuf;

use dotty::utils;
use dotty::error::DottyError;

#[test]
fn test_resolve_tilde_simple() {
    let correct_path = home_dir();
    let result = utils::resolve_tilde(PathBuf::from("~")).ok();

    assert_eq!(correct_path, result);
}

#[test]
fn test_resolve_tilde_complex() {
    let mut correct_path = home_dir();
    let result = utils::resolve_tilde(PathBuf::from("~/a/longer/path")).ok();

    if let Some(p) = correct_path {
        p.push("a/longer/path");
        assert_eq!(p, result);
    } else {
        assert_eq!(None, result)
    }
}
*/

#![feature(plugin)]
#![plugin(indoc)]

#[macro_use(defer)]
extern crate scopeguard;
#[macro_use(assert_cli)]
extern crate assert_cli;

use std::fs;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::env::current_dir;


// Utility functions
pub fn to_absolute(path : &PathBuf) -> Result<PathBuf, io::Error> {
    if path.is_relative() {
        let mut c_dir = try!(current_dir());
        c_dir.push(path);
        Ok(c_dir)
    } else {
        Ok(path.clone())
    }
}

#[test]
fn simple_install() {
    // Create a setup to test installation on
    defer!(fs::remove_dir_all("cli_tests1").unwrap());
    
    let m_config = indoc!("
        name: test
        links:
            - file1:cli_tests1/test_dest/file1_link
    ");
    let d_config = indoc!("
        local_data_dir: \"cli_tests1/dotty_data\"
    ");
    
    fs::create_dir_all("cli_tests1/test").unwrap();
    fs::create_dir_all("cli_tests1/test_dest").unwrap();
    fs::File::create("cli_tests1/test/file1").unwrap();
    let mut module = fs::File::create("cli_tests1/test/module.yml").unwrap();
    module.write(m_config.as_bytes()).unwrap();
    let mut config = fs::File::create("cli_tests1/config.yaml").unwrap();
    config.write(d_config.as_bytes()).unwrap();
    
    // Make sure the command runs
    assert_cli!("cargo", &["run", "--", "-c", "cli_tests1/config.yaml", "install", "cli_tests1/test"] => Success).unwrap();
    
    // Check the resulting file system to see if it's correct 
    let sym_path = Path::new("cli_tests1/test_dest/file1_link");
    assert!(&sym_path.exists());
    assert_eq!(sym_path.read_link().unwrap(), to_absolute(&PathBuf::from("cli_tests1/test/file1")).unwrap());
}

#[test]
fn recursive_install() {
    // Create a setup to test installation on
    defer!(fs::remove_dir_all("cli_tests2").unwrap());
    
    let m_config = indoc!("
        name: test
        links:
            - dir1:cli_tests2/test_dest/dir1
    ");
    let d_config = indoc!("
        local_data_dir: \"cli_tests2/dotty_data\"
    ");
    
    fs::create_dir_all("cli_tests2/test/dir1").unwrap();
    fs::create_dir_all("cli_tests2/test_dest/dir1").unwrap();
    fs::File::create("cli_tests2/test/dir1/file1").unwrap();
    fs::File::create("cli_tests2/test/dir1/file2").unwrap();
    let mut module = fs::File::create("cli_tests2/test/module.yml").unwrap();
    module.write(m_config.as_bytes()).unwrap();
    let mut config = fs::File::create("cli_tests2/config.yaml").unwrap();
    config.write(d_config.as_bytes()).unwrap();
    
    // Make sure the command runs
    assert_cli!("cargo", &["run", "--", "-c", "cli_tests2/config.yaml", "install", "cli_tests2/test"] => Success).unwrap();
    
    // Check the resulting file system to see if it's correct 
    let sym_paths = (Path::new("cli_tests2/test_dest/dir1/file1"), Path::new("cli_tests2/test_dest/dir1/file2"));
    assert!(&sym_paths.0.exists());
    assert!(&sym_paths.1.exists());
    assert_eq!(sym_paths.0.read_link().unwrap(), to_absolute(&PathBuf::from("cli_tests2/test/dir1/file1")).unwrap());
    assert_eq!(sym_paths.1.read_link().unwrap(), to_absolute(&PathBuf::from("cli_tests2/test/dir1/file2")).unwrap());
}

#[test]
fn remove() {
    // Create a setup to test installation on
    defer!(fs::remove_dir_all("cli_tests3").unwrap());
    
    let m_config = indoc!("
        name: test
        links:
            - dir1:cli_tests3/test_dest/dir1
    ");
    let d_config = indoc!("
        local_data_dir: \"cli_tests3/dotty_data\"
    ");
    
    fs::create_dir_all("cli_tests3/test/dir1").unwrap();
    fs::create_dir_all("cli_tests3/test_dest/dir1").unwrap();
    fs::File::create("cli_tests3/test/dir1/file1").unwrap();
    fs::File::create("cli_tests3/test/dir1/file2").unwrap();
    let mut module = fs::File::create("cli_tests3/test/module.yml").unwrap();
    module.write(m_config.as_bytes()).unwrap();
    let mut config = fs::File::create("cli_tests3/config.yaml").unwrap();
    config.write(d_config.as_bytes()).unwrap();
    
    // Make sure the command runs
    assert_cli!("cargo", &["run", "--", "-c", "cli_tests3/config.yaml", "install", "cli_tests3/test"] => Success).unwrap();
    
    // Check the resulting file system to see if it's correct 
    let sym_paths = (Path::new("cli_tests3/test_dest/dir1/file1"), Path::new("cli_tests3/test_dest/dir1/file2"));
    assert!(&sym_paths.0.exists());
    assert!(&sym_paths.1.exists());
    assert_eq!(sym_paths.0.read_link().unwrap(), to_absolute(&PathBuf::from("cli_tests3/test/dir1/file1")).unwrap());
    assert_eq!(sym_paths.1.read_link().unwrap(), to_absolute(&PathBuf::from("cli_tests3/test/dir1/file2")).unwrap());
    
    
    // Test remove - NOTE: This is broken right now because I can't have assert_cli ignore the program's output
    assert_cli!("cargo", &["run", "--", "-c", "cli_tests3/config.yaml", "remove", "test"] => Success).unwrap();
    
    let sym_paths = (Path::new("cli_tests3/test_dest/dir1/file1"), Path::new("cli_tests3/test_dest/dir1/file2"));
    assert!(! &sym_paths.0.exists());
    assert!(! &sym_paths.1.exists());
}