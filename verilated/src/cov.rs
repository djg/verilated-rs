// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{io, ffi::CString, path::Path};

mod verilated_cov {
    extern "C" {
        #[link_name = "\u{1}__ZN12VerilatedCov5writeEPKc"]
        pub fn write(filenamep: *const ::std::os::raw::c_char);
        #[link_name = "\u{1}__ZN12VerilatedCov5clearEv"]
        pub fn clear();
        #[link_name = "\u{1}__ZN12VerilatedCov13clearNotMatchEPKc"]
        pub fn clear_non_match(matchp: *const ::std::os::raw::c_char);
        #[link_name = "\u{1}__ZN12VerilatedCov4zeroEv"]
        pub fn zero();
    }
}

/// Return default filename
pub fn default_filename() -> &'static str { "coverage.dat" }

/// Write all coverage data to a file
pub fn write(path: impl AsRef<Path>) -> io::Result<()> {
    use std::os::unix::ffi::OsStrExt;
    let path = CString::new(path.as_ref().as_os_str().as_bytes())?;
    unsafe {
        verilated_cov::write(path.as_ptr())
    };
    Ok(())
}

/// Clear coverage points (and call delete on all items)
pub fn clear() {
    unsafe { verilated_cov::clear() };
}

/// Clear items not matching the provided string `s`
pub fn clear_non_match(s: &str) -> io::Result<()> {
    let s = CString::new(s.as_bytes())?;
    unsafe {
        verilated_cov::clear_non_match(s.as_ptr())
    };
    Ok(())
}

/// Zero coverage points
pub fn zero() {
    unsafe {
        verilated_cov::zero()
    };
}
