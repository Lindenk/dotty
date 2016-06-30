
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