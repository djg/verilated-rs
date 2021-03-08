// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    env,
    path::{Path, PathBuf},
};
use verilator::find_verilator_root;

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}

fn generate_bindings(root: impl AsRef<Path>, prefix: &str, class: &str) {
    let root = root.as_ref();
    let builder = bindgen::Builder::default()
        .clang_args(&["-xc++", "-std=gnu++14"])
        .clang_arg(format!("-I{}/include", root.to_string_lossy()))
        .header(root.join(format!("include/{}.h", prefix)).to_string_lossy())
        .whitelist_type(class)
        .opaque_type("std.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    let bindings = builder.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join(format!("{}.rs", prefix)))
        .expect("Couldn't write bindings!");
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
            .define("VM_TRACE", "1")
            .define("VM_TRACE_FST", "0");
        cfg.include(&include).include(include.join("vltstd"));
        cfg.files(files);
        cfg.compile("verilated_rt");

        generate_bindings(&root, "verilated_vcd_c", "VerilatedVcd");
    } else {
        fail("Failed to find `${VERILATOR_ROOT}`.  Please set `VERILATOR_ROOT` environment variable or ensure `verilator` is in `PATH`.");
    }
}
