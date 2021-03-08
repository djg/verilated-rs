// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::TimeUnit;
use std::{ffi::CString, path::Path, ptr};

mod verilated_vcd {
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/verilated_vcd_c.rs"));

    // rust-bindgen doesn't know how to generate this binding, so do it
    // manually.
    extern "C" {
        #[doc = " Write one cycle of dump data"]
        #[link_name = "\u{1}__ZN14VerilatedTraceI12VerilatedVcdE4dumpEm"]
        pub fn VerilatedVcd_dump(this: *mut VerilatedVcd, time: u64);
    }

    impl VerilatedVcd {
        #[inline]
        pub unsafe fn dump(&mut self, time: u64) {
            VerilatedVcd_dump(self, time)
        }
    }
}

/// Create a VCD dump file in C standalone (no SystemC) simulations.
/// Also derived for use in SystemC simulations.
/// Thread safety: Unless otherwise indicated, every function is VL_MT_UNSAFE_ONE
pub struct Vcd {
    sptrace: verilated_vcd::VerilatedVcd,
}

impl Vcd {
    unsafe fn alloc() -> *mut Self {
        use ::std::alloc::{alloc, Layout};
        let layout = Layout::new::<verilated_vcd::VerilatedVcd>();
        let __tmp = alloc(layout) as *mut verilated_vcd::VerilatedVcd;
        verilated_vcd::VerilatedVcd_VerilatedVcd(__tmp, ptr::null_mut());
        __tmp as *mut Self
    }

    pub fn new() -> Box<Self> {
        unsafe {
            let raw_ffi = Self::alloc();
            Box::from_raw(raw_ffi)
        }
    }

    /// Open a new VCD file
    /// This includes a complete header dump each time it is called,
    /// just as if this object was deleted and reconstructed.
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
        if self.sptrace.m_isOpen {
            Ok(())
        } else {
            Err(())
        }
    }

    /// Continue a VCD dump by rotating to a new file name
    /// The header is only in the first file created, this allows
    /// "cat" to be used to combine the header plus any number of data files.
    pub fn open_next(&mut self, inc_filename: bool) {
        unsafe {
            self.sptrace.openNext(inc_filename);
        }
    }

    /// Set size in megabytes after which new file should be created
    pub fn rollover_mb(&mut self, rollover_mb: usize) {
        self.sptrace.m_rolloverMB = rollover_mb as u64;
    }

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

impl Drop for Vcd {
    fn drop(&mut self) {
        unsafe {
            self.sptrace.destruct();
        }
    }
}
