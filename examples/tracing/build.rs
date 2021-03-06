// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::{Context, Result};

use std::{env, fs, path::PathBuf};
use verilator::gen::Verilator;

fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR").context("Accessing OUT_DIR env var")?;
    println!("Targeting OUT_DIR={}", out_dir);
    let out_dir = PathBuf::from(out_dir);
    fs::remove_dir_all(&out_dir).context("Remove OUT_DIR")?;
    fs::create_dir_all(&out_dir).context("Creating OUT_DIR")?;

    // Generate CPP from Verilog
    let mut verilator = Verilator::default();
    verilator
        .with_coverage(false)
        .with_trace(true)
        .files(&["rtl/sub.sv", "rtl/top.v"])
        .build("top");

    Ok(())
}
