#![feature(proc_macro)]

extern crate verilated;
extern crate verilated_module;
use verilated_module::module;
use verilated::test_bench::TestBench;

#[module(top)]
pub struct Top {
    #[port(clock)] pub clk_i: bool,
    #[port(reset)] pub rst_i: bool,
    #[port(output)] pub count_o: [bool; 4],
}

fn main() {
    let mut tb = TestBench::<Top>::init(|core, tick_count| {
        if tick_count > 10 {
            return false;
        }

        println!("{}: count_o = {}", tick_count, core.count_o());

        true
    });

    while !tb.done() {
        tb.tick();
    }
}
