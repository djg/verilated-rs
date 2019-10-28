extern crate verilated;
extern crate verilated_module;

use std::time::Duration;

use verilated_module::module;

#[module(top)]
pub struct Top {
    #[port(clock)]
    pub clk_i: bool,
    #[port(reset)]
    pub rst_i: bool,
    #[port(output)]
    pub count_o: [bool; 4],
}

fn main() {
    let mut tb: Top = Top::default();
    tb.eval();
    tb.eval();

    tb.open_trace("counter.vcd", 99).unwrap();

    let mut clocks: u64 = 0;
    while tb.count_o() < 10 {
        if clocks == 0 {
            tb.reset_toggle();
        } else if clocks == 2 {
            tb.reset_toggle();
        }

        tb.clock_toggle();
        tb.eval();
        tb.trace_at(Duration::from_nanos(20 * clocks));

        tb.clock_toggle();
        tb.eval();
        tb.trace_at(Duration::from_nanos(20 * clocks + 10));

        println!("{}: count_o = {}", clocks, tb.count_o());

        clocks += 1;
    }
    tb.trace_at(Duration::from_nanos(20 * clocks));

    tb.finish();
}
