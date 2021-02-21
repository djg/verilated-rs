// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::TimeUnit;
use std::{ffi::CString, path::Path};

mod std_cpp {
    // TODO: This all depends upon the STL headers supplied by the system C++
    // compiler. We should bindgen this on each platform.

    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    #[allow(non_camel_case_types)]
    pub struct std___compressed_pair {
        pub _address: u8,
    }

    #[repr(C)]
    #[allow(non_camel_case_types)]
    pub struct std_basic_string {
        pub _base: u8,
        pub __r_: std___compressed_pair,
    }

    #[allow(non_camel_case_types)]
    pub type std_string = std_basic_string;

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_allocator_traits {
        pub _address: u8,
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    #[allow(non_camel_case_types)]
    pub struct std___make_tree_node_types {
        pub _address: u8,
    }

    #[repr(C)]
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    pub struct std___tree {
        pub __begin_node_: std___make_tree_node_types,
        pub __pair1_: std___compressed_pair,
        pub __pair3_: std___compressed_pair,
    }

    #[repr(C)]
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    pub struct std_map {
        pub __tree_: std___tree,
    }

    #[repr(C)]
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    pub struct std___vector_base {
        pub _base: u8,
        pub __begin_: std_allocator_traits,
        pub __end_: std_allocator_traits,
        pub __end_cap_: std___compressed_pair,
    }

    #[repr(C)]
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    pub struct std_vector {
        pub _base: std___vector_base,
    }
}

mod verilated_vcd {
    use super::std_cpp::*;
    use std::{
        mem::MaybeUninit,
        os::raw::{c_char, c_int, c_void},
    };

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct VerilatedAssertOneThread {
        pub _address: u8,
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct VerilatedVcdFile {
        pub vtable_: *const c_void,
        pub m_fd: c_int,
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct VerilatedTrace {
        pub vtable_: *const c_void,
        pub m_sigs_oldvalp: *mut u32,
        pub m_timeLastDump: u64,
        pub m_initCbs: std_vector,
        pub m_fullCbs: std_vector,
        pub m_chgCbs: std_vector,
        pub m_cleanupCbs: std_vector,
        pub m_fullDump: bool,
        pub m_nextCode: u32,
        pub m_numSignals: u32,
        pub m_maxBits: u32,
        pub m_moduleName: std_string,
        pub m_scopeEscape: c_char,
        pub m_timeRes: f64,
        pub m_timeUnit: f64,
        pub m_assertOne: VerilatedAssertOneThread,
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct VerilatedVcd {
        pub _base: VerilatedTrace,
        pub m_filep: *mut VerilatedVcdFile,
        pub m_fileNewed: bool,
        pub m_isOpen: bool,
        pub m_evcd: bool,
        pub m_filename: std_string,
        pub m_rolloverMB: u64,
        pub m_modDepth: c_int,
        pub m_wrBufp: *mut c_char,
        pub m_wrFlushp: *mut c_char,
        pub m_writep: *mut c_char,
        pub m_wrChunkSize: u64,
        pub m_wroteBytes: u64,
        pub m_suffixes: std_vector,
        pub m_suffixesp: *const c_char,
        pub m_namemapp: *mut std_map,
    }

    extern "C" {
        #[link_name = "\u{1}__ZN12VerilatedVcdC1EP16VerilatedVcdFile"]
        pub fn constructor(this: *mut VerilatedVcd, filep: *mut VerilatedVcdFile);
        #[link_name = "\u{1}__ZN12VerilatedVcdD1Ev"]
        pub fn destructor(this: *mut VerilatedVcd);
        #[link_name = "\u{1}__ZN12VerilatedVcd4openEPKc"]
        pub fn open(this: *mut VerilatedVcd, filename: *const c_char);
        #[link_name = "\u{1}__ZN12VerilatedVcd8openNextEb"]
        pub fn open_next(this: *mut VerilatedVcd, incFilename: bool);
        #[link_name = "\u{1}__ZN12VerilatedVcd5flushEv"]
        pub fn flush(this: *mut VerilatedVcd);
        #[link_name = "\u{1}__ZN14VerilatedTraceI12VerilatedVcdE4dumpEm"]
        pub fn dump(this: *mut VerilatedVcd, time: u64);
    }

    impl VerilatedVcd {
        pub unsafe fn new(filep: *mut VerilatedVcdFile) -> Self {
            let mut tmp = MaybeUninit::uninit();
            constructor(tmp.as_mut_ptr(), filep);
            tmp.assume_init()
        }

        pub unsafe fn destruct(&mut self) {
            destructor(self)
        }

        pub unsafe fn open(&mut self, filename: *const c_char) {
            open(self, filename)
        }

        pub fn is_open(&self) -> bool {
            self.m_isOpen
        }

        pub fn open_next(&mut self, inc_filename: bool) {
            unsafe { open_next(self, inc_filename) }
        }

        pub unsafe fn flush(&mut self) {
            flush(self)
        }

        pub unsafe fn dump(&mut self, time: u64) {
            dump(self, time)
        }
    }

    impl Default for VerilatedVcd {
        fn default() -> Self {
            unsafe { Self::new(std::ptr::null_mut()) }
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
    /// Open a new VCD file
    /// This includes a complete header dump each time it is called,
    /// just as if this object was deleted and reconstructed.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, ()> {
        #[cfg(unix)]
        fn path_to_bytes<P: AsRef<Path>>(path: P) -> Vec<u8> {
            use std::os::unix::ffi::OsStrExt;
            path.as_ref().as_os_str().as_bytes().to_vec()
        }

        let filename = unsafe { CString::from_vec_unchecked(path_to_bytes(path)) };
        let mut sptrace = verilated_vcd::VerilatedVcd::default();
        unsafe {
            sptrace.open(filename.as_ptr());
        }
        if sptrace.is_open() {
            Ok(Vcd { sptrace })
        } else {
            Err(())
        }
    }

    /// Continue a VCD dump by rotating to a new file name
    /// The header is only in the first file created, this allows
    /// "cat" to be used to combine the header plus any number of data files.
    pub fn open_next(&mut self, inc_filename: bool) {
        self.sptrace.open_next(inc_filename);
    }

    /// Set size in megabytes after which new file should be created
    pub fn rollover_mb(&mut self, rollover_mb: usize) {
        self.sptrace.m_rolloverMB = rollover_mb as u64;
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
