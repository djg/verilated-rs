// Copyright (C) 2018 Dan Glastonbury <dan.glastonbury@gmail.com>

use std::ffi::CString;
use std::env;
use std::os::unix::ffi::OsStringExt;

pub trait Module {
    /// Evaluate the model.  Application must call when inputs change.
    fn eval(&mut self);
    /// Simulation complete, run final blocks.
    fn finish(&mut self);

    fn clock_up(&mut self);
    fn clock_down(&mut self);

    fn reset_up(&mut self);
    fn reset_down(&mut self);
}

type TickFn<'a, UUT> = FnMut(&mut UUT, usize) -> bool + 'a;

pub struct TestBench<'a, UUT>
where
    UUT: Module + Default,
{
    core: UUT,
    tick_count: usize,
    tick_fn: Box<TickFn<'a, UUT>>,
}

impl<'a, UUT> TestBench<'a, UUT>
where
    UUT: Module + Default,
{
    pub fn init<F>(f: F) -> Self
    where
        F: FnMut(&mut UUT, usize) -> bool + 'a,
    {
        let args: Vec<CString> = env::args_os()
            .map(|a| unsafe { CString::from_vec_unchecked(a.into_vec()) })
            .collect();
        Self::init_with_command_args(args, f)
    }

    pub fn init_with_command_args<F>(args: Vec<CString>, f: F) -> Self
    where
        F: FnMut(&mut UUT, usize) -> bool + 'a,
    {
        super::command_args(args);
        TestBench {
            core: UUT::default(),
            tick_count: 0,
            tick_fn: Box::new(f),
        }
    }

    pub fn reset(&mut self) {
        self.core.reset_up();
        (self.tick_fn)(&mut self.core, self.tick_count);
        self.core.reset_down();
    }

    pub fn tick(&mut self) {
        // Increment our own internal time reference
        self.tick_count += 1;

        // Make sure any combinatorial logic depending upon
        // inputs that may have changed before we call tick()
        // have settled before the rising edge of the clock.
        self.core.clock_down();
        self.core.eval();

        // *** Toggle the clock ***

        // Rising edge
        self.core.clock_up();
        self.core.eval();

        // Falling edge
        self.core.clock_down();
        self.core.eval();

        if !(self.tick_fn)(&mut self.core, self.tick_count) {
            super::set_finish();
            self.core.finish();
        }
    }

    pub fn done(&mut self) -> bool {
        super::got_finish()
    }
}
