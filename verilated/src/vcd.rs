use std::ffi::{CStr, CString};
use std::io;
use std::path::Path;

pub enum VcdC {}

mod ffi {
    use super::VcdC;
    use std::os::raw::{c_char, c_int};

    extern "C" {
        pub fn verilatedvcdc_new() -> *mut VcdC;
        pub fn verilatedvcdc_delete(vcd: *mut VcdC);
        pub fn verilatedvcdc_is_open(vcd: *mut VcdC) -> c_int;
        pub fn verilatedvcdc_open(vcd: *mut VcdC, filename: *const c_char);
        pub fn verilatedvcdc_open_next(vcd: *mut VcdC, inc_filename: c_int);
        pub fn verilatedvcdc_rollover_mb(vcd: *mut VcdC, rolloverMB: usize);
        pub fn verilatedvcdc_close(vcd: *mut VcdC);
        pub fn verilatedvcdc_flush(vcd: *mut VcdC);
        pub fn verilatedvcdc_dump(vcd: *mut VcdC, timeui: u64);
        pub fn verilatedvcdc_set_time_unit(vcd: *mut VcdC, unit: *const c_char);
        pub fn verilatedvcdc_set_time_resolution(vcd: *mut VcdC, unit: *const c_char);
    }
}

#[cfg(unix)]
fn cstr(path: &Path) -> io::Result<CString> {
    use std::os::unix::ffi::OsStrExt;
    Ok(CString::new(path.as_os_str().as_bytes())?)
}

#[cfg(not(unix))]
fn cstr(path: &Path) -> io::Result<CString> {
    Ok(CString::new(
        path.to_str()
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("path is not valid utf-8"),
            ))?
            .as_bytes(),
    )?)
}

pub struct Vcd(pub *mut VcdC);

impl Vcd {
    fn _open(&mut self, path: &CStr) -> io::Result<()> {
        unsafe {
            ffi::verilatedvcdc_open(self.0, path.as_ptr());
            if ffi::verilatedvcdc_is_open(self.0) == 0 {
                return Err(io::ErrorKind::Other.into());
            }
            let time_unit = CString::new("1ns").unwrap();
            ffi::verilatedvcdc_set_time_unit(self.0, time_unit.as_ptr() as *const _);
            ffi::verilatedvcdc_set_time_resolution(self.0, time_unit.as_ptr() as *const _);
        }

        Ok(())
    }

    pub fn open<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let path = cstr(path.as_ref())?;
        self._open(&path)
    }

    pub fn open_next(&mut self, inc_filename: i32) {
        unsafe { ffi::verilatedvcdc_open_next(self.0, inc_filename) }
    }

    pub fn rollover_mb(&mut self, rollover_mb: usize) {
        unsafe { ffi::verilatedvcdc_rollover_mb(self.0, rollover_mb) }
    }

    pub fn flush(&mut self) {
        unsafe { ffi::verilatedvcdc_flush(self.0) }
    }

    pub fn dump(&mut self, nanos: u64) {
        unsafe { ffi::verilatedvcdc_dump(self.0, nanos) }
    }
}

impl Default for Vcd {
    fn default() -> Vcd {
        let ptr = unsafe { ffi::verilatedvcdc_new() };
        if ptr.is_null() {
            panic!("Failed to allocate VerilatedVcdC");
        }
        Vcd(ptr)
    }
}

impl Drop for Vcd {
    fn drop(&mut self) {
        unsafe {
            ffi::verilatedvcdc_close(self.0);
            ffi::verilatedvcdc_delete(self.0);
        }
    }
}
