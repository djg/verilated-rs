// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub mod api;
pub mod cov;
pub mod fst;
pub mod vcd;
pub mod vpi;

pub use crate::api::*;
pub use verilated_macro::verilated;

extern "C" {
    fn sc_time_stamp() -> f64;
}

// Re-export to a binding of C friendly name `sc_time_stamp`
#[no_mangle]
extern "C" fn _Z13sc_time_stampv() -> f64 {
    unsafe { sc_time_stamp() }
}
