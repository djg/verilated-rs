// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use verilated::TimeUnit;

#[test]
fn time_unit() {
    macro_rules! test {
        ($x:ident, $y:expr) => {{
            verilated::set_timeunit(TimeUnit::$x);
            assert_eq!(verilated::timeunit(), TimeUnit::$x);
            assert_eq!(verilated::timeunit_string().to_bytes(), $y);
        }};
    }

    test!(_1s, b"1s");
    test!(_100ms, b"100ms");
    test!(_10ms, b"10ms");
    test!(_1ms, b"1ms");
    test!(_100ms, b"100ms");
    test!(_10ms, b"10ms");
    test!(_1ms, b"1ms");
    test!(_100us, b"100us");
    test!(_10us, b"10us");
    test!(_1us, b"1us");
    test!(_100ns, b"100ns");
    test!(_10ns, b"10ns");
    test!(_1ns, b"1ns");
    test!(_100ps, b"100ps");
    test!(_10ps, b"10ps");
    test!(_1ps, b"1ps");
    test!(_100fs, b"100fs");
    test!(_10fs, b"10fs");
    test!(_1fs, b"1fs");
}
