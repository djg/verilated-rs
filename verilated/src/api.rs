// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    convert::TryFrom,
    ffi::{c_void, CStr, CString, OsStr},
    num::NonZeroI32,
    os::raw::{c_char, c_int},
    panic::{catch_unwind, AssertUnwindSafe},
};

mod verilated {

    use super::*;

    #[repr(C)]
    pub struct Serialized {
        pub debug: c_int,
        pub calc_unused_sigs: bool,
        pub got_finish: bool,
        pub assert_on: bool,
        pub fatal_on_vpi_error: bool,
        pub timeunit: i8,
        pub timeprecision: i8,
        pub error_count: c_int,
        pub error_limit: c_int,
        pub rand_reset: c_int,
        pub rand_seed: c_int,
        pub rand_seed_epoch: u32,
    }

    extern "C" {
        #[link_name = "_ZN9Verilated3s_sE"]
        pub static s_s: Serialized;
    }

    #[repr(C)]
    pub struct NonSerialized {
        pub prof_threads_start: u64,
        pub prof_threads_window: u32,
        pub prof_threads_filenamep: *const c_char,
    }

    extern "C" {
        #[link_name = "_ZN9Verilated4s_nsE"]
        pub static s_ns: NonSerialized;
    }

    #[repr(C)]
    pub struct CommandArgValues {
        //VerilatedMutex m_argMutex;
        pub argc: c_int,
        pub argv: *const *const c_char,
    }

    extern "C" {
        #[link_name = "_ZN9Verilated6s_argsE"]
        pub static s_args: CommandArgValues;
    }

    pub type VoidPCb = unsafe extern "C" fn(*mut c_void);

    extern "C" {
        #[link_name = "_ZN9Verilated9randResetEi"]
        pub fn set_rand_reset(val: c_int);
        #[link_name = "_ZN9Verilated8randSeedEi"]
        pub fn set_rand_seed(val: c_int);
        #[link_name = "_ZN9Verilated17randSeedDefault64Ev"]
        pub fn rand_seed_default64() -> u64;
        #[link_name = "_ZN9Verilated5debugEi"]
        pub fn set_debug(level: c_int);
        #[link_name = "_ZN9Verilated14calcUnusedSigsEb"]
        pub fn set_calc_unused_sigs(flag: bool);
        #[link_name = "_ZN9Verilated10errorCountEi"]
        pub fn set_error_count(val: c_int);
        #[link_name = "_ZN9Verilated13errorCountIncEv"]
        pub fn error_count_inc();
        #[link_name = "_ZN9Verilated10errorLimitEi"]
        pub fn set_error_limit(val: c_int);
        #[link_name = "_ZN9Verilated9gotFinishEb"]
        pub fn set_got_finish(flag: bool);
        #[link_name = "_ZN9Verilated8assertOnEb"]
        pub fn set_assert_on(flag: bool);
        #[link_name = "_ZN9Verilated15fatalOnVpiErrorEb"]
        pub fn set_fatal_on_vpi_error(flag: bool);
        #[link_name = "_ZN9Verilated8timeunitEi"]
        pub fn set_timeunit(val: c_int);
        #[link_name = "_ZN9Verilated14timeunitStringEv"]
        pub fn timeunit_string() -> *const c_char;
        #[link_name = "_ZN9Verilated13timeprecisionEi"]
        pub fn set_timeprecision(val: c_int);
        #[link_name = "_ZN9Verilated19timeprecisionStringEv"]
        pub fn timeprecision_string() -> *const c_char;
        #[link_name = "_ZN9Verilated16profThreadsStartEm"]
        pub fn set_prof_threads_start(flag: u64);
        #[link_name = "_ZN9Verilated17profThreadsWindowEm"]
        pub fn set_prof_threads_window(flag: u64);
        #[link_name = "_ZN9Verilated20profThreadsFilenamepEPKc"]
        pub fn set_prof_threads_filenamep(flagp: *const c_char);
        #[link_name = "_ZN9Verilated10addFlushCbEPFvPvES0_"]
        pub fn add_flush_cb(cb: VoidPCb, datap: *mut c_void);
        #[link_name = "_ZN9Verilated13removeFlushCbEPFvPvES0_"]
        pub fn remove_flush_cb(cb: VoidPCb, datap: *mut c_void);
        #[link_name = "_ZN9Verilated17runFlushCallbacksEv"]
        pub fn run_flush_callbacks();
        #[link_name = "_ZN9Verilated9addExitCbEPFvPvES0_"]
        pub fn add_exit_cb(cb: VoidPCb, datap: *mut c_void);
        #[link_name = "_ZN9Verilated12removeExitCbEPFvPvES0_"]
        pub fn remove_exit_cb(cb: VoidPCb, datap: *mut c_void);
        #[link_name = "_ZN9Verilated16runExitCallbacksEv"]
        pub fn run_exit_callbacks();
        #[link_name = "_ZN9Verilated11commandArgsEiPPKc"]
        pub fn command_args(argc: c_int, argv: *const *const c_char);
        //#[link_name = ""]
        //pub fn command_args_add(argc: c_int, argv: &c_char);
        #[link_name = "_ZN9Verilated20commandArgsPlusMatchEPKc"]
        pub fn command_args_plus_match(prefixp: *const c_char) -> *const c_char;
        #[link_name = "_ZN9Verilated11productNameEv"]
        pub fn product_name() -> *const c_char;
        #[link_name = "_ZN9Verilated14productVersionEv"]
        pub fn product_version() -> *const c_char;
        #[link_name = "_ZN9Verilated5mkdirEPKc"]
        pub fn mkdir(dirname: *const c_char);
        #[link_name = "_ZN9Verilated7quiesceEv"]
        pub fn quiesce();
        #[link_name = "_ZN9Verilated13internalsDumpEv"]
        pub fn internals_dump();
        #[link_name = "_ZN9Verilated10scopesDumpEv"]
        pub fn scopes_dump();
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum RandomMode {
    AllZeros,
    AllBits,
    Randomize,
}

/// Select initial value of otherwise uninitialized signals.
///
/// 0 = Set to zeros
/// 1 = Set all bits to one
/// 2 = Randomize all bits
pub fn set_rand_reset(val: RandomMode) {
    let val = match val {
        RandomMode::AllZeros => 0,
        RandomMode::AllBits => 1,
        RandomMode::Randomize => 2,
    };
    unsafe {
        verilated::set_rand_reset(val);
    }
}

/// Return randReset value
pub fn rand_reset() -> RandomMode {
    match unsafe { verilated::s_s.rand_reset } {
        0 => RandomMode::AllZeros,
        1 => RandomMode::AllBits,
        2 => RandomMode::Randomize,
        _ => unreachable!("Invalid internal verilated state"),
    }
}

pub fn set_rand_seed(val: Option<NonZeroI32>) {
    let val = val.map(NonZeroI32::get).unwrap_or_default();
    unsafe { verilated::set_rand_seed(val) }
}

pub fn rand_seed() -> Option<NonZeroI32> {
    NonZeroI32::new(unsafe { verilated::s_s.rand_seed })
}

pub fn rand_seed_epoch() -> u32 {
    unsafe { verilated::s_s.rand_seed_epoch }
}

pub fn rand_seed_default_64() -> u64 {
    unsafe { verilated::rand_seed_default64() }
}

/// Enable debug of internal verilated code
pub fn set_debug(level: i32) {
    unsafe { verilated::set_debug(level) }
}

/// Return debug value
#[cfg(debug_assertions)]
pub fn debug() -> i32 {
    unsafe { verilated::s_s.debug }
}

#[cfg(not(debug_assertions))]
pub fn debug() -> i32 {
    0
}

/// Enable calculation of unused signals
pub fn set_calc_unused_sigs(flag: bool) {
    unsafe { verilated::set_calc_unused_sigs(flag) }
}

pub fn calc_unused_sigs() -> bool {
    unsafe { verilated::s_s.calc_unused_sigs }
}

/// Current number of errors/assertions
pub fn set_error_count(val: i32) {
    unsafe { verilated::set_error_count(val) }
}

pub fn error_count_inc() {
    unsafe { verilated::error_count_inc() }
}

pub fn error_count() -> i32 {
    unsafe { verilated::s_s.error_count }
}

/// Set number of errors/assertions before stop
pub fn set_error_limit(val: i32) {
    unsafe { verilated::set_error_limit(val) }
}

pub fn error_limit() -> i32 {
    unsafe { verilated::s_s.error_limit }
}

/// Did the simulation $finish?
pub fn set_finish(flag: bool) {
    unsafe {
        verilated::set_got_finish(flag);
    }
}

/// Return if got a $finish
pub fn got_finish() -> bool {
    unsafe { verilated::s_s.got_finish }
}

/// Allow traces to at some point be enabled (disables some optimizations)
pub fn trace_ever_on(on: bool) {
    if on {
        set_calc_unused_sigs(on);
    }
}

/// Enable/disable assertions
pub fn set_assert_on(flag: bool) {
    unsafe { verilated::set_assert_on(flag) }
}

pub fn verilated_assert_on() -> bool {
    unsafe { verilated::s_s.assert_on }
}

/// Enable/disable vpi fatal
pub fn set_fatal_on_vpi_error(flag: bool) {
    unsafe { verilated::set_fatal_on_vpi_error(flag) }
}

pub fn fatal_on_vpi_error() -> bool {
    unsafe { verilated::s_s.fatal_on_vpi_error }
}

/// Time handling
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TimeUnit {
    _100s,
    _10s,
    _1s,
    _100ms,
    _10ms,
    _1ms,
    _100us,
    _10us,
    _1us,
    _100ns,
    _10ns,
    _1ns,
    _100ps,
    _10ps,
    _1ps,
    _100fs,
    _10fs,
    _1fs,
}

impl TryFrom<i8> for TimeUnit {
    type Error = ();

    fn try_from(val: i8) -> Result<TimeUnit, Self::Error> {
        use TimeUnit::*;
        match val {
            2 => Ok(_100s),
            1 => Ok(_10s),
            0 => Ok(_1s),
            -1 => Ok(_100ms),
            -2 => Ok(_10ms),
            -3 => Ok(_1ms),
            -4 => Ok(_100us),
            -5 => Ok(_10us),
            -6 => Ok(_1us),
            -7 => Ok(_100ns),
            -8 => Ok(_10ns),
            -9 => Ok(_1ns),
            -10 => Ok(_100ps),
            -11 => Ok(_10ps),
            -12 => Ok(_1ps),
            -13 => Ok(_100fs),
            -14 => Ok(_10fs),
            -15 => Ok(_1fs),
            _ => Err(()),
        }
    }
}

impl From<TimeUnit> for c_int {
    fn from(val: TimeUnit) -> Self {
        use TimeUnit::*;
        match val {
            _100s => 2,
            _10s => 1,
            _1s => 0,
            _100ms => -1,
            _10ms => -2,
            _1ms => -3,
            _100us => -4,
            _10us => -5,
            _1us => -6,
            _100ns => -7,
            _10ns => -8,
            _1ns => -9,
            _100ps => -10,
            _10ps => -11,
            _1ps => -12,
            _100fs => -13,
            _10fs => -14,
            _1fs => -15,
        }
    }
}

impl From<TimeUnit> for f64 {
    fn from(val: TimeUnit) -> Self {
        use TimeUnit::*;
        match val {
            _100s => 1e2,
            _10s => 1e1,
            _1s => 1e0,
            _100ms => 1e-1,
            _10ms => 1e-2,
            _1ms => 1e-3,
            _100us => 1e-4,
            _10us => 1e-5,
            _1us => 1e-6,
            _100ns => 1e-7,
            _10ns => 1e-8,
            _1ns => 1e-9,
            _100ps => 1e-10,
            _10ps => 1e-11,
            _1ps => 1e-12,
            _100fs => 1e-13,
            _10fs => 1e-14,
            _1fs => 1e-15,
        }
    }
}

pub fn timeunit() -> TimeUnit {
    TimeUnit::try_from(unsafe { -verilated::s_s.timeunit })
        .expect("Invalid internal verilated state")
}

pub fn timeunit_string() -> &'static CStr {
    unsafe { CStr::from_ptr(verilated::timeunit_string()) }
}

pub fn set_timeunit(val: TimeUnit) {
    unsafe { verilated::set_timeunit(val.into()) }
}

pub fn timeprecision() -> TimeUnit {
    TimeUnit::try_from(unsafe { -verilated::s_s.timeprecision })
        .expect("Invalid internal verilated state")
}

pub fn timeprecision_string() -> &'static CStr {
    unsafe { CStr::from_ptr(verilated::timeprecision_string()) }
}

pub fn set_timeprecision(val: TimeUnit) {
    unsafe { verilated::set_timeprecision(val.into()) }
}

/// --prof-threads related settings
pub fn set_prof_threads_start(flag: u64) {
    unsafe { verilated::set_prof_threads_start(flag) }
}

pub fn prof_threads_start() -> u64 {
    unsafe { verilated::s_ns.prof_threads_start }
}

pub fn set_prof_threads_window(flag: u64) {
    unsafe { verilated::set_prof_threads_window(flag) }
}

pub fn prof_threads_window() -> u32 {
    unsafe { verilated::s_ns.prof_threads_window }
}

pub fn set_prof_threads_filenamep(flagp: &CStr) {
    unsafe { verilated::set_prof_threads_filenamep(flagp.as_ptr()) }
}
pub fn prof_threads_filenamep() -> Option<&'static CStr> {
    let ptr = unsafe { verilated::s_ns.prof_threads_filenamep };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(ptr) })
    }
}

struct Thunk {
    cb: Box<dyn FnMut()>,
}

impl Thunk {
    fn new(cb: impl FnMut() + 'static) -> *mut Self {
        let thunk = Thunk { cb: Box::new(cb) };
        Box::into_raw(Box::new(thunk))
    }

    fn call(&mut self) {
        catch_unwind(AssertUnwindSafe(|| {
            (*self.cb)();
        }))
        .expect("Callback panic'd");
    }
}

extern "C" fn thunk_cb(raw: *mut c_void) {
    let thunk = unsafe { &mut *(raw as *mut Thunk) };
    thunk.call();
}

pub struct Handle(
    *mut Thunk,
    unsafe extern "C" fn(verilated::VoidPCb, *mut c_void),
);

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            self.1(thunk_cb, self.0 as *mut _);
        }
        let _drop = unsafe { Box::from_raw(self.0) };
    }
}

/// Callbacks to run on global flush
pub fn add_flush_cb(cb: impl FnMut() + 'static) -> Handle {
    let thunk = Thunk::new(cb);
    unsafe { verilated::add_flush_cb(thunk_cb, thunk as *mut _) }
    Handle(thunk, verilated::remove_flush_cb)
}

pub fn run_flush_callbacks() {
    unsafe { verilated::run_flush_callbacks() }
}

/// Callbacks to run prior to termination
pub fn add_exit_cb(cb: impl FnMut() + 'static) -> Handle {
    let thunk = Thunk::new(cb);
    unsafe { verilated::add_exit_cb(thunk_cb, thunk as *mut _) }
    Handle(thunk, verilated::remove_exit_cb)
}

pub fn run_exit_callbacks() {
    unsafe { verilated::run_exit_callbacks() }
}

/// Record command line arguments, for retrieval by $test$plusargs/$value$plusargs,
/// and for parsing +verilator+ run-time arguments.
/// This should be called before the first model is created.
pub fn set_command_args(args: &[&CStr]) {
    let args = args.iter().map(|a| a.as_ptr()).collect::<Vec<_>>();
    unsafe { verilated::command_args(args.len() as _, args.as_ptr() as *const *const c_char) }
}

pub fn command_args() -> &'static verilated::CommandArgValues {
    unsafe { &verilated::s_args }
}

/// Match plusargs with a given prefix. Returns static char* valid only for a single call
pub fn command_args_plus_match(prefix: &CStr) -> Option<&CStr> {
    unsafe { opt(verilated::command_args_plus_match(prefix.as_ptr())) }
}

/// Produce name & version for (at least) VPI
pub fn product_name() -> &'static CStr {
    unsafe { CStr::from_ptr(verilated::product_name()) }
}

pub fn product_version() -> &'static CStr {
    unsafe { CStr::from_ptr(verilated::product_version()) }
}

/// Convenience OS utilities
pub fn mkdir(dirname: &OsStr) {
    use std::os::unix::ffi::OsStrExt;
    let dirname = CString::new(dirname.as_bytes()).expect("dirname is invalid as CString");
    unsafe { verilated::mkdir(dirname.as_ptr()) }
}

/// When multithreaded, quiesce the model to prepare for trace/saves/coverage
/// This may only be called when no locks are held.
pub fn quiesce() {
    unsafe { verilated::quiesce() }
}

/// For debugging, print much of the Verilator internal state.
/// The output of this function may change in future
/// releases - contact the authors before production use.
pub fn internals_dump() {
    unsafe {
        verilated::internals_dump();
    }
}

/// For debugging, print text list of all scope names with
/// dpiImport/Export context.  This function may change in future
/// releases - contact the authors before production use.
pub fn scopes_dump() {
    unsafe {
        verilated::scopes_dump();
    }
}

fn opt(c_str: *const c_char) -> Option<&'static CStr> {
    if c_str.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(c_str) })
    }
}
