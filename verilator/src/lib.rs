#[cfg(feature = "gen")]
extern crate cc;
#[cfg(feature = "module")]
extern crate fnv;
#[cfg(feature = "module")]
extern crate syntex_syntax as syntax;

#[cfg(feature = "gen")]
pub mod gen;
#[cfg(feature = "module")]
pub mod module;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn check_verilator_bin(path: &Path) -> bool {
    path.join("verilator_bin").is_file()
}

fn check_verilator_root(root: &Path) -> bool {
    root.join("include/verilated.cpp").is_file()
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
