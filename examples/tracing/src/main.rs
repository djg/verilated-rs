// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Based upon Verilater example module: make_tracing_c
// SPDX-License-Identifier: CC0-1.0

use std::sync::atomic::{self, AtomicU64};
use verilated::verilated;

macro_rules! atomic {
    ($atom:ident += $inc:expr) => {
        $atom.fetch_add($inc, atomic::Ordering::SeqCst)
    };

    ($atom:ident) => {
        $atom.load(atomic::Ordering::SeqCst)
    };
}
#[verilated]
pub struct Top;

// Extend the verilated wrapper with custom methods
impl Top {
    pub fn clk_toggle(&mut self) {
        self.clk = if self.clk == 0 { 1 } else { 0 };
    }
}

// Current simulation time (64-bit unsigned)
static MAIN_TIME: AtomicU64 = AtomicU64::new(0);

// Called by $time in Verilog
#[no_mangle]
pub extern "C" fn sc_time_stamp() -> f64 {
    // Note does conversion to real, to match SystemC
    MAIN_TIME.load(atomic::Ordering::SeqCst) as f64
}

fn main() {
    // Set debug level, 0 is off, 9 is highest presently used
    // May be overridden by commandArgs
    verilated::set_debug(0);

    // Randomization reset policy
    // May be overridden by commandArgs
    verilated::set_rand_reset(verilated::RandomMode::Randomize);

    // Verilator must compute traced signals
    verilated::set_trace_ever_on(true);

    // Pass arguments so Verilated code can see them, e.g. $value$plusargs
    // This needs to be called before you create any model
    verilated::set_command_args(std::env::args());

    // Create logs/ directory in case we have traces to put under it
    verilated::mkdir("logs");

    // Construct the Verilated model, from Vtop.h generated from Verilating "top.v"
    // Using unique_ptr is similar to "Vtop* top = new Vtop" then deleting at end
    let mut top = Top::new("TOP").expect("Failed to allocate Top module");

    // Set some inputs
    top.reset_l = 1;
    top.clk = 0;
    top.in_small = 1;
    top.in_quad = 0x1234;
    top.in_wide[0] = 0x11111111;
    top.in_wide[1] = 0x22222222;
    top.in_wide[2] = 0x3;

    // Simulate until $finish
    while !verilated::got_finish() {
        atomic!(MAIN_TIME += 1); // Time passes...

        // Toggle a fast (time/2 period) clock
        top.clk_toggle();

        // Toggle control signals on an edge that doesn't correspond
        // to where the controls are sampled; in this example we do
        // this only on a negedge of clk, because we know
        // reset is not sampled there.
        if top.clk == 0 {
            let curr_time = atomic!(MAIN_TIME);
            top.reset_l = if curr_time > 1 && curr_time < 10 {
                0 // Assert reset
            } else {
                1 // Deassert reset
            };
            // Assign some other inputs
            top.in_quad += 0x12;
        }

        // Evaluate model
        // (If you have multiple models being simulated in the same
        // timestep then instead of eval(), call eval_step() on each, then
        // eval_end_step() on each.)
        top.eval();

        // Read outputs
        println!(
            "[{}] clk={} rstl={} iquad={:x} -> oquad={:x} wide={:x}_{:08x}_{:08x}",
            atomic!(MAIN_TIME),
            top.clk,
            top.reset_l,
            top.in_quad,
            top.out_quad,
            top.out_wide[2],
            top.out_wide[1],
            top.out_wide[0]
        );
    }

    // Final model cleanup
    top.finalize();

    //  Coverage analysis (since test passed)
    #[cfg(verilated = "coverage")]
    write_coverage();
}

#[cfg(verilated = "coverage")]
fn write_coverage() {
    verilated::mkdir("logs");
    verilated::cov::write("logs/coverage.dat").expect("Failed to write coverage");
}
