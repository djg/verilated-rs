#[cfg(feature = "module")]
extern crate fnv;
extern crate regex;
#[cfg(feature = "module")]
extern crate syn;

#[cfg(feature = "gen")]
pub mod gen;
#[cfg(feature = "module")]
pub mod module;

use regex::Regex;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::from_utf8;

fn check_verilator_bin(path: &Path) -> bool {
    path.join("verilator_bin").is_file()
}

fn check_verilator_root(root: &Path) -> bool {
    root.join("include/verilated.cpp").is_file()
}

/// Parse the version of verilator on `$PATH`
pub fn verilator_version() -> Option<(u32, u32)> {
    let re = Regex::new(r"^Verilator (\d{1}).(\d{3})").expect("Failed to create version regex");
    let output = Command::new("verilator_bin")
        .arg("--version")
        .output()
        .ok()?;
    let stdout = from_utf8(&output.stdout).ok()?;
    let captures = re.captures(stdout)?;
    let major = captures[1].parse::<u32>().ok()?;
    let minor = captures[2].parse::<u32>().ok()?;
    Some((major, minor))
}

pub fn find_verilator_root() -> Option<PathBuf> {
    env::split_paths(&env::var_os("PATH").unwrap_or_default())
        .find(|p| check_verilator_bin(p.as_ref()))
        .and_then(|p| {
            let mut cmd = Command::new(p.join("verilator_bin"));
            cmd.arg("--getenv").arg("VERILATOR_ROOT");

            cmd.output().ok().map(|output| {
                // Report what verilator says is ${VERILATOR_ROOT}
                PathBuf::from(String::from_utf8_lossy(&output.stdout).trim())
            })
        })
        .or_else(|| env::var("VERILATOR_ROOT").map(PathBuf::from).ok())
        .and_then(|p| {
            if check_verilator_root(p.as_ref()) {
                Some(p)
            } else {
                None
            }
        })
}
