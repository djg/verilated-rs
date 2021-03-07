// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use log::{info, warn};
use std::{
    error,
    ffi::{CStr, CString},
    fmt::{self, Display},
    mem::MaybeUninit,
    ops::Deref,
    os::raw::{c_char, c_int, c_void},
    ptr,
};

mod user;

mod verilated_vpi {
    use std::os::raw::c_int;

    extern "C" {
        #[link_name = "\u{1}__ZN12VerilatedVpi12callTimedCbsEv"]
        pub fn call_timed_cbs();
        #[link_name = "\u{1}__ZN12VerilatedVpi12callValueCbsEv"]
        pub fn call_value_cbs() -> bool;
        #[link_name = "\u{1}__ZN12VerilatedVpi7callCbsEj"]
        pub fn call_cbs(reason: c_int) -> bool;
        #[link_name = "\u{1}__ZN12VerilatedVpi14cbNextDeadlineEv"]
        pub fn cb_next_deadline() -> u64;
    }
}

/// Call timed callbacks
/// Users should call this from their main loops
pub fn call_timed_cbs() {
    unsafe {
        verilated_vpi::call_timed_cbs();
    }
}

/// Call value based callbacks
/// Users should call this from their main loops
pub fn call_value_cbs() -> bool {
    unsafe { verilated_vpi::call_value_cbs() }
}

/// Call callbacks of arbitrary types
/// Users can call this from their application code
pub fn call_cbs(reason: Reason) -> bool {
    unsafe { verilated_vpi::call_cbs(reason.as_ffi()) }
}

/// Returns time of the next registered VPI callback, or
/// !0 if none are registered
pub fn cb_next_deadline() -> u64 {
    unsafe { verilated_vpi::cb_next_deadline() }
}

macro_rules! init {
    ($fn:ident($($param:expr),*)) => {
        unsafe {
            let mut __tmp = MaybeUninit::uninit();
            user::$fn($($param,)* __tmp.as_mut_ptr());
            __tmp.assume_init()
        }
    };

    ($fn:ident) => {
        unsafe {
            let mut __tmp = MaybeUninit::uninit();
            user::$fn(__tmp.as_mut_ptr());
            __tmp.assume_init()
        }
    };

    (ASSERT $fn:ident) => {
        unsafe {
            let mut __tmp = MaybeUninit::uninit();
            let result = user::$fn(__tmp.as_mut_ptr());
            debug_assert_eq!(result, 1);
            __tmp.assume_init()
        }
    };
}

pub trait AsFfi {
    type Target;
    fn as_ffi(&self) -> Self::Target;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectType {
    /// always procedure
    Always = 1,
    /* quasi-continuous assignment */
    AssignStmt,
    /* procedural assignment */
    Assignment,
    /* block statement */
    Begin,
    /* case statement */
    Case,
    /* case statement item */
    CaseItem,
    Constant,      /* numerical constant or string literal */
    ContAssign,    /* continuous assignment */
    Deassign,      /* deassignment statement */
    DefParam,      /* defparam */
    DelayControl,  /* delay statement (e.g., #10) */
    Disable,       /* named block disable statement */
    EventControl,  /* wait on event, e.g., @e */
    EventStmt,     /* event trigger, e.g., ->e */
    For,           /* for statement */
    Force,         /* force statement */
    Forever,       /* forever statement */
    Fork,          /* fork-join block */
    FuncCall,      /* function call */
    Function,      /* function */
    Gate,          /* primitive gate */
    If,            /* if statement */
    IfElse,        /* if-else statement */
    Initial,       /* initial procedure */
    IntegerVar,    /* integer variable */
    InterModPath,  /* intermodule wire delay */
    Iterator,      /* iterator */
    IODecl,        /* input/output declaration */
    Memory,        /* behavioral memory */
    MemoryWord,    /* single word of memory */
    ModPath,       /* module path for path delays */
    Module,        /* module instance */
    NamedBegin,    /* named block statement */
    NamedEvent,    /* event variable */
    NamedFork,     /* named fork-join block */
    Net,           /* scalar or vector net */
    NetBit,        /* bit of vector net */
    NullStmt,      /* a semicolon. Ie. #10 ; */
    Operation,     /* behavioral operation */
    ParamAssign,   /* module parameter assignment */
    Parameter,     /* module parameter */
    PartSelect,    /* part-select */
    PathTerm,      /* terminal of module path */
    Port,          /* module port */
    PortBit,       /* bit of vector module port */
    PrimTerm,      /* primitive terminal */
    RealVar,       /* real variable */
    Reg,           /* scalar or vector reg */
    RegBit,        /* bit of vector reg */
    Release,       /* release statement */
    Repeat,        /* repeat statement */
    RepeatControl, /* repeat control in an assign stmt */
    SchedEvent,    /* vpi_put_value() event */
    SpecParam,     /* specparam */
    /* transistor switch */
    Switch,
    /* system function call */
    SysFuncCall,
    /* system task call */
    SysTaskCall,
    /* UDP state table entry */
    TableEntry,
    /* task */
    Task,
    /* task call */
    TaskCall,
    /* timing check */
    Tchk,
    /* terminal of timing check */
    TchkTerm,
    /* time variable */
    TimeVar,
    /* simulation event queue */
    TimeQueue,
    /* user-defined primitive */
    Udp,
    /* UDP definition */
    UdpDefn,
    /* user-defined system task/function */
    UserSystf,
    /* variable array selection */
    VarSelect,
    /* wait statement */
    Wait,
    /* multidimensional net */
    While,

    // object types added with 1364-2001
    /* attribute of an object */
    Attribute,
    /* Bit-select of parameter, var select */
    BitSelect,
    /* callback object */
    Callback,
    /* Delay term which is a load or driver */
    DelayTerm,
    /* Delay object within a net */
    DelayDevice,
    /* reentrant task/func frame */
    Frame,
    /* gate instance array */
    GateArray,
    /* module instance array */
    ModuleArray,
    /* vpiprimitiveArray type */
    PrimitiveArray,
    /* multidimensional net */
    NetArray,
    /* range declaration */
    Range,
    /* multidimensional reg */
    RegArray,
    /* switch instance array */
    SwitchArray,
    /* UDP instance array */
    UdpArray,
    /* Bit of a vector continuous assignment */
    ContAssignBit,
    /* multidimensional named event */
    NamedEventArray,

    // object types added with 1364-2005
    /* Indexed part-select object */
    IndexedPartSelect,
    /* array of generated scopes */
    GenScopeArray,
    /* A generated scope */
    GenScope,
    /* Object used to instantiate gen scopes */
    GenVar,
}

macro_rules! handle {
    ($handle:expr) => {
        if $handle.is_null() {
            None
        } else {
            Some(Handle($handle))
        }
    };
}

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub file: String,
    pub line: i32,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}: {}", self.file, self.line, self.message)
    }
}

impl Error {
    fn new(error_info: &user::s_vpi_error_info) -> Self {
        let message = string_or(error_info.message, "<null>");
        let file = string_or(error_info.file, "<null>");
        Error {
            message,
            file,
            line: error_info.line,
        }
    }
}

impl error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

fn chk_err() -> Result<()> {
    if unsafe { user::vpi_chk_error(ptr::null()) == 0 } {
        return Ok(());
    }

    let error_info = init!(vpi_chk_error);

    let err = Error::new(&error_info);
    match ErrorLevel::from(error_info.level) {
        ErrorLevel::Notice => {
            info!("{}", err);
            Ok(())
        }
        ErrorLevel::Warning => {
            warn!("{}", err);
            Ok(())
        }
        ErrorLevel::Error | ErrorLevel::System | ErrorLevel::Internal => Err(err),
    }
}

/// Callback reason
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reason {
    /* Simulation related */
    ValueChange = 1,
    Stmt,
    Force,
    Release, /* Time related */
    AtStartOfSimTime,
    ReadWriteSynch,
    ReadOnlySynch,
    NextSimTime,
    AfterDelay,

    // Action related
    EndOfCompile,
    StartOfSimulation,
    EndOfSimulation,
    Error,
    TchkViolation,
    StartOfSave,
    EndOfSave,
    StartOfRestart,
    EndOfRestart,
    StartOfReset,
    EndOfReset,
    EnterInteractive,
    ExitInteractive,
    InteractiveScopeChange,
    UnresolvedSystf,

    // Added with 1364-2001
    Assign,
    Deassign,
    Disable,
    PLIError,
    Signal,

    // Added with 1364-2005
    NBASynch,
    AtEndOfSimTime,
}

impl AsFfi for Reason {
    type Target = c_int;
    fn as_ffi(&self) -> Self::Target {
        use Reason::*;
        match *self {
            ValueChange => user::cbValueChange,
            Stmt => user::cbStmt,
            Force => user::cbForce,
            Release => user::cbRelease,
            AtStartOfSimTime => user::cbAtStartOfSimTime,
            ReadWriteSynch => user::cbReadWriteSynch,
            ReadOnlySynch => user::cbReadOnlySynch,
            NextSimTime => user::cbNextSimTime,
            AfterDelay => user::cbAfterDelay,

            // Action related
            EndOfCompile => user::cbEndOfCompile,
            StartOfSimulation => user::cbStartOfSimulation,
            EndOfSimulation => user::cbEndOfSimulation,
            Error => user::cbError,
            TchkViolation => user::cbTchkViolation,
            StartOfSave => user::cbStartOfSave,
            EndOfSave => user::cbEndOfSave,
            StartOfRestart => user::cbStartOfRestart,
            EndOfRestart => user::cbEndOfRestart,
            StartOfReset => user::cbStartOfReset,
            EndOfReset => user::cbEndOfReset,
            EnterInteractive => user::cbEnterInteractive,
            ExitInteractive => user::cbExitInteractive,
            InteractiveScopeChange => user::cbInteractiveScopeChange,
            UnresolvedSystf => user::cbUnresolvedSystf,

            // Added with 1364-2001
            Assign => user::cbAssign,
            Deassign => user::cbDeassign,
            Disable => user::cbDisable,
            PLIError => user::cbPLIError,
            Signal => user::cbSignal,

            // Added with 1364-2005
            NBASynch => user::cbNBASynch,
            AtEndOfSimTime => user::cbAtEndOfSimTime,
        }
    }
}

pub struct Handle(user::vpiHandle);

impl Handle {
    fn as_ffi(&self) -> user::vpiHandle {
        self.0
    }
}

impl Deref for Handle {
    type Target = HandleRef;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.0 as *const HandleRef) }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                user::vpi_release_handle(self.0);
            }
            self.0 = ptr::null_mut();
        }
    }
}

#[derive(Clone, Copy)]
pub struct HandleRef {
    __priv: (),
}

impl AsFfi for HandleRef {
    type Target = user::vpiHandle;
    fn as_ffi(&self) -> Self::Target {
        self as *const HandleRef as Self::Target
    }
}

//pub fn register_cb(cb_data_p: self::user::p_cb_data) -> Handle {}
//pub fn remove_cb(cb_obj: HandleRef) -> Result<(), ()> { 0 }

/* for obtaining handles */

pub fn handle_by_name<'a>(
    name: impl AsRef<str>,
    scope: impl Into<Option<&'a HandleRef>>,
) -> Option<Handle> {
    use user::vpi_handle_by_name;

    let name = CString::new(name.as_ref()).expect("name is invalid CString");
    let scope = scope.into().map_or(ptr::null_mut(), |s| s.as_ffi());
    handle!(unsafe { vpi_handle_by_name(name.as_ptr() as *const _, scope) })
}

pub fn handle_by_index(object: HandleRef, index: i32) -> Result<Option<Handle>> {
    use user::vpi_handle_by_index;

    let handle = unsafe { vpi_handle_by_index(object.as_ffi(), index) };
    chk_err()?;
    Ok(handle!(handle))
}

/* for traversing relationships */
pub enum HandleKind {
    LeftRange,
    RightRange,
    Index,
    Scope,
    Parent,
}

impl AsFfi for HandleKind {
    type Target = c_int;
    fn as_ffi(&self) -> Self::Target {
        use HandleKind::*;
        match *self {
            LeftRange => user::vpiLeftRange,
            RightRange => user::vpiRightRange,
            Index => user::vpiIndex,
            Scope => user::vpiScope,
            Parent => user::vpiParent,
        }
    }
}

impl HandleRef {
    pub fn handle(&self, kind: HandleKind) -> Option<Handle> {
        use user::vpi_handle;

        // vpi_handle only generates warnings and returns NULL on failure.
        let handle = handle!(unsafe { vpi_handle(kind.as_ffi(), self.as_ffi()) });
        let _ = chk_err();
        handle
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IterKind {
    MemoryWord,
    Range,
    Reg,
    Module,
}

impl IterKind {
    fn as_ffi(&self) -> c_int {
        use IterKind::*;
        match *self {
            MemoryWord => user::vpiMemoryWord,
            Range => user::vpiRange,
            Reg => user::vpiReg,
            Module => user::vpiModule,
        }
    }
}

pub struct Iter {
    iter: Handle,
}

impl Iter {
    fn new<'a>(kind: IterKind, object: impl Into<Option<&'a HandleRef>>) -> Option<Self> {
        use user::vpi_iterate;

        let object = object.into().map_or(ptr::null_mut(), AsFfi::as_ffi);
        // vpi_iterator only generates warnings and returns NULL on failure.
        let iter = unsafe { vpi_iterate(kind.as_ffi(), object) };
        let _ = chk_err();
        let iter = handle!(iter)?;
        Some(Self { iter })
    }
}

impl Iterator for Iter {
    type Item = Handle;

    fn next(&mut self) -> Option<Self::Item> {
        use user::vpi_scan;
        let next = handle!(unsafe { vpi_scan(self.iter.as_ffi()) });
        if next.is_none() {
            // Don't need to free the iterator handle according to Verilog
            // specification. Set to null ptr so drop will skip calling
            // `vpi_release_handle`.
            self.iter.0 = ptr::null_mut();
        }
        next
    }
}

macro_rules! impl_iters {
    ($($name: ident => $kind: ident),*) => {
        impl HandleRef {
            $(pub fn $name(&self) -> Option<Iter> {
                Iter::new(IterKind::$kind, self)
            })*
        }
    };
    ($($name:ident => $kind:ident,)*) => {
        impl_iters!($($name => $kind),*)
    }
}

impl_iters! {
    memory_words => MemoryWord,
    ranges => Range,
    regs => Reg,
    modules => Module
}

pub fn top_modules() -> Option<Iter> {
    Iter::new(IterKind::Module, None)
}

/* for processing properties */
pub enum Property {}

impl HandleRef {
    //pub fn int32(&self, property: Property) -> i32 {}
    //pub fn cstr(&self, property: Property) -> &'static CStr {}
}

/* value processing */

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scalar {
    _0,
    _1,
    Z,
    X,
    H,
    L,
    DontCare,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Scalar(Scalar),
    Integer(i32),
    Real(f64),
    Time(),
    Vector(),
    Strength(*const c_void),
    Misc(),
}

impl HandleRef {
    //pub fn value(&self, value_p: self::user::p_vpi_value) -> Value {}
    // pub fn put_value(
    //     &self,
    //     value_p: p_vpi_value,
    //     time_p: p_vpi_time,
    //     flags: PLI_INT32,
    // ) -> vpiHandle {}
}

/* time processing */

pub fn sim_time() -> u64 {
    let vpi_time = init!(vpi_get_time(ptr::null_mut()));
    (vpi_time.high as u64) << 32 | vpi_time.low as u64
}

pub fn sim_time_scaled() -> f64 {
    let vpi_time = init!(vpi_get_time(ptr::null_mut()));
    vpi_time.real
}

impl HandleRef {
    pub fn time(&self) -> f64 {
        let vpi_time = init!(vpi_get_time(self.as_ffi()));
        vpi_time.real
    }
}

/* I/O routines */

// pub fn mcd_open(fileName: *const PLI_BYTE8) -> PLI_UINT32;
// pub fn mcd_close(mcd: PLI_UINT32) -> PLI_UINT32;
// pub fn mcd_name(cd: PLI_UINT32) -> *const PLI_BYTE8;
// pub fn mcd_printf(mcd: PLI_UINT32, format: *const PLI_BYTE8, ...) -> PLI_INT32;
// pub fn printf(format: *const PLI_BYTE8, ...) -> PLI_INT32;

/* utility routines */

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ErrorLevel {
    Notice = 1,
    Warning,
    Error,
    System,
    Internal,
}

impl From<c_int> for ErrorLevel {
    fn from(value: c_int) -> Self {
        use ErrorLevel::*;
        #[allow(non_upper_case_globals)]
        match value {
            user::vpiNotice => Notice,
            user::vpiWarning => Warning,
            user::vpiError => Error,
            user::vpiSystem => System,
            user::vpiInternal => Internal,
            n => panic!("invalid ErrorLevel: {}", n),
        }
    }
}

pub fn check_error() -> std::result::Result<(), ErrorLevel> {
    use user::vpi_chk_error;
    let error_level = unsafe { vpi_chk_error(std::ptr::null()) };
    if error_level == 0 {
        Ok(())
    } else {
        Err(ErrorLevel::from(error_level))
    }
}

pub struct VlogInfo {
    pub args: Vec<String>,
    pub product: String,
    pub version: String,
}

pub fn vlog_info() -> VlogInfo {
    let vlog_info = init!(ASSERT vpi_get_vlog_info);

    let args = (0..vlog_info.argc)
        .into_iter()
        .map(|n| string_or(unsafe { *vlog_info.argv.offset(n as _) }, "<null>"))
        .collect::<Vec<_>>();
    let product = string_or(vlog_info.product, "unknown");
    let version = string_or(vlog_info.version, "unknown");
    VlogInfo {
        args,
        product,
        version,
    }
}

// TODO: Needs c_variadic https://github.com/rust-lang/rust/issues/44930
//pub fn vpi_vprintf(format: *const PLI_BYTE8, ap: va_list) -> PLI_INT32;
// TODO: Needs c_variadic https://github.com/rust-lang/rust/issues/44930
//pub fn vpi_mcd_vprintf(mcd: PLI_UINT32, format: *const PLI_BYTE8, ap: va_list)
//    -> PLI_INT32;

// pub fn flush() -> PLI_INT32;
// pub fn mcd_flush(mcd: PLI_UINT32) -> PLI_INT32;
// pub fn control(operation: PLI_INT32, ...) -> PLI_INT32;
// pub fn handle_by_multi_index(
//     obj: vpiHandle,
//     num_index: PLI_INT32,
//     index_array: *const PLI_INT32,
// ) -> vpiHandle;

fn string_or(c_str: *const c_char, default: &str) -> String {
    if !c_str.is_null() {
        unsafe { CStr::from_ptr(c_str).to_string_lossy().into_owned() }
    } else {
        default.to_owned()
    }
}
