extern crate verilator;

use verilator::ModuleGenerator;

fn main() {
    ModuleGenerator::new().generate("src/main.rs");
}
