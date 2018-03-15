# verilated - Verilator Porcelain for Rust

A build dependency for running `verilator` to generate Rust bindings to Verilog code.

## Motivation

## Usage

```toml
# Cargo.toml
[build-dependencies]
verilator = { version = "0.1", features = ["gen", "module"] }
```

```rust
// build.rs
extern crate verilator;

use verilator::gen::Verilator;
use verilator::module::ModuleGenerator;

fn main() {
    ...
    // Generate CPP shim from Rust
    ModuleGenerator::new().generate("src/main.rs");
    
    ...
    // Generate CPP from Verilog, ...
    Verilator::new()
        .file("rtl/top.v");
    // ... and from ModuleGenerator output ...
        .file("top.cpp")
    // ... compile and link into crate.
        .build("top");
}
```

## Example

```rust
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
```

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
