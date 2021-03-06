// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(feature = "gen")]
pub mod gen;

use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

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
