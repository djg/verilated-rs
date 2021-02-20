// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::path::PathBuf;
use verilator::find_verilator_root;

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}

fn main() {
    if let Some(root) = find_verilator_root() {
        let include = root.join("include");

        let files = &[
            "verilated.cpp",
            "verilated_cov.cpp",
            "verilated_dpi.cpp",
            "verilated_save.cpp",
            "verilated_vcd_c.cpp",
            "verilated_vpi.cpp",
        ];

        let files: Vec<PathBuf> = files.iter().map(|&p| include.join(p)).collect();

        let mut cfg = cc::Build::new();
        let tool = cfg.get_compiler();
        cfg.cpp(true);
        if tool.is_like_gnu() {
            cfg.flag("-std=gnu++14")
                .flag("-faligned-new")
                .flag("-fbracket-depth=4096")
                .flag("-fcf-protection=none")
                .flag("-Qunused-arguments")
                .flag("-Wno-bool-operation")
                .flag("-Wno-tautological-bitwise-compare")
                .flag("-Wno-parentheses-equality")
                .flag("-Wno-sign-compare")
                .flag("-Wno-uninitialized")
                .flag("-Wno-unused-parameter")
                .flag("-Wno-unused-variable")
                .flag("-Wno-shadow")
                .flag("-Os");
        }
        cfg.define("VM_COVERAGE", "0")
            .define("VM_SC", "0")
            .define("VM_TRACE", "0")
            .define("VM_TRACE_FST", "0");
        cfg.include(&include).include(include.join("vltstd"));
        cfg.files(files);
        cfg.compile("verilated_rt");
    } else {
        fail("Failed to find `${VERILATOR_ROOT}`.  Please set `VERILATOR_ROOT` environment variable or ensure `verilator` is in `PATH`.");
    }
}
