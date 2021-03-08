// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::TimeUnit;
use std::{ffi::CString, path::Path, ptr};

mod verilated_fst {
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/verilated_fst_c.rs"));

    // rust-bindgen doesn't know how to generate this binding, so do it
    // manually.
    extern "C" {
        #[doc = " Write one cycle of dump data"]
        #[link_name = "\u{1}__ZN14VerilatedTraceI12VerilatedFstE4dumpEm"]
        pub fn VerilatedFst_dump(this: *mut VerilatedFst, time: u64);
    }

    impl VerilatedFst {
        #[inline]
        pub unsafe fn dump(&mut self, time: u64) {
            VerilatedFst_dump(self, time)
        }
    }
}

/// Create a VCD dump file in C standalone (no SystemC) simulations.
/// Also derived for use in SystemC simulations.
/// Thread safety: Unless otherwise indicated, every function is VL_MT_UNSAFE_ONE
pub struct Fst {
    sptrace: verilated_fst::VerilatedFst,
}

impl Fst {
    unsafe fn alloc() -> *mut Self {
        use ::std::alloc::{alloc, Layout};
        let layout = Layout::new::<verilated_fst::VerilatedFst>();
        let __tmp = alloc(layout) as *mut verilated_fst::VerilatedFst;
        verilated_fst::VerilatedFst_VerilatedFst(__tmp, ptr::null_mut());
        __tmp as *mut Self
    }

    pub fn new() -> Box<Self> {
        unsafe {
            let raw_ffi = Self::alloc();
            Box::from_raw(raw_ffi)
        }
    }

    /// Is file open?
    #[inline]
    pub fn is_open(&self) -> bool {
        !self.sptrace.m_fst.is_null()
    }

    /// Open a new FST file
    pub fn open(&mut self, path: impl AsRef<Path>) -> Result<(), ()> {
        #[cfg(unix)]
        fn path_to_cstring(path: &Path) -> CString {
            use std::os::unix::ffi::OsStrExt;
            CString::new(path.as_os_str().as_bytes()).expect("Path invalid as CString")
        }

        let filename = path_to_cstring(path.as_ref());
        unsafe {
            self.sptrace.open(filename.as_ptr());
        }
        if self.is_open() {
            Ok(())
        } else {
            Err(())
        }
    }

    // Close dump
    pub fn close(&mut self) {
        unsafe { self.sptrace.close() }
    }

    /// Flush dump
    pub fn flush(&mut self) {
        unsafe {
            self.sptrace.flush();
        }
    }

    /// Write one cycle of dump data
    pub fn dump(&mut self, time: u64) {
        unsafe {
            self.sptrace.dump(time);
        }
    }

    /// Set time units (s/ms, defaults to ns)
    /// For Verilated models, these propagate from the Verilated default --timeunit
    pub fn set_time_unit(&mut self, unit: TimeUnit) {
        self.sptrace._base.m_timeUnit = unit.into();
    }

    /// Set time resolution (s/ms, defaults to ns)
    /// For Verilated models, these propagate from the Verilated default --timeunit
    pub fn set_time_resolution(&mut self, unit: TimeUnit) {
        self.sptrace._base.m_timeRes = unit.into();
    }
}

impl Drop for Fst {
    fn drop(&mut self) {
        unsafe {
            self.sptrace.destruct();
        }
    }
}
