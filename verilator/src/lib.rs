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
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

fn check_verilator_root(root: &Path) -> bool {
    root.join("verilator").is_file()
}

pub fn find_verilator_root() -> Option<PathBuf> {
    env::var_os("VERILATOR_ROOT")
        .map(|p| PathBuf::from(p).join("bin"))
        .into_iter()
        .chain(env::split_paths(&env::var_os("PATH")
            .unwrap_or(OsString::new())))
        .find(|p| check_verilator_root(p.as_ref()))
        .and_then(|p| {
            let mut cmd = Command::new(p.join("verilator"));
            cmd.arg("--getenv").arg("VERILATOR_ROOT");

            match cmd.output() {
                // Report what verilator says is ${VERILATOR_ROOT}
                Ok(output) => Some(PathBuf::from(
                    String::from_utf8_lossy(&output.stdout).trim(),
                )),
                Err(..) => p.parent().map(|p| p.join("share")),
            }
        })
}
