use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Standard {
    Verilog1995,
    Verilog2001,
    Verilog2005,
    SystemVerilog2005,
    SystemVerilog2009,
    SystemVerilog2012,
}

/// Builder style configuration for running verilator.
pub struct Verilator {
    out_dir: Option<PathBuf>,
    root: Option<PathBuf>,
    files: Vec<(PathBuf, Option<Standard>)>,
    module_directories: Vec<PathBuf>,
    coverage: bool,
    trace: bool,
    optimized: bool, 
    suppress_warnings: Vec<String>,
}

impl Verilator {
    pub fn out_dir<P>(&mut self, out: P) -> &mut Verilator
    where
        P: AsRef<Path>,
    {
        self.out_dir = Some(out.as_ref().to_path_buf());
        self
    }

    pub fn root<P>(&mut self, root: P) -> &mut Verilator
    where
        P: AsRef<Path>,
    {
        self.root = Some(root.as_ref().to_path_buf());
        self
    }

    fn _file(&mut self, p: &Path, s: Option<Standard>) -> &mut Verilator {
        self.files.push((p.to_path_buf(), s));
        self
    }

    pub fn file<P>(&mut self, p: P) -> &mut Verilator
    where
        P: AsRef<Path>,
    {
        self._file(p.as_ref(), None)
    }

    pub fn file_with_standard<P>(&mut self, p: P, s: Standard) -> &mut Verilator
    where
        P: AsRef<Path>,
    {
        self._file(p.as_ref(), Some(s))
    }

    pub fn files<P>(&mut self, p: P) -> &mut Verilator
    where
        P: IntoIterator,
        P::Item: AsRef<Path>,
    {
        for file in p {
            self.file(file);
        }
        self
    }

    /// Add a directory to the `-y` or path to search for modules
    ///
    /// # Example
    ///
    /// ```ignore
    /// use std::path::Path;
    ///
    /// let module_path = Path::new("/path/to/modules");
    ///
    /// verilator::Verilator::default()
    ///     .file("top.v")
    ///     .module(module_path)
    ///     .build(...);
    /// ```
    pub fn module<P>(&mut self, dir: P) -> &mut Verilator
    where
        P: AsRef<Path>,
    {
        self.module_directories.push(dir.as_ref().to_path_buf());
        self
    }

    pub fn with_coverage(&mut self, t: bool) -> &mut Verilator {
        self.coverage = t;
        self
    }

    pub fn with_trace(&mut self, t: bool) -> &mut Verilator {
        self.trace = t;
        self
    }

    pub fn with_performance_optimizations(&mut self, t: bool) -> &mut Verilator {
        self.optimized = t;
        self
    }

    pub fn warn_width(&mut self, t: bool) -> &mut Verilator {
        if !t {
            self.suppress_warnings.push("width".to_string());
        }
        self
    }

    pub fn no_warn(&mut self, warning: &str) -> &mut Verilator {
        self.suppress_warnings.push(warning.to_ascii_lowercase());
        self
    }

    pub fn build(&mut self, top_module: &str) -> PathBuf {
        let dst = self
            .out_dir
            .clone()
            .unwrap_or_else(|| PathBuf::from(getenv_unwrap("OUT_DIR")));

        // Determine ${VERILATOR_ROOT}
        let verilator_exe = self.find_verilator_exe();
        let mut cmd = Command::new(verilator_exe.clone());
        cmd.arg("--getenv").arg("VERILATOR_ROOT");

        println!("running: {:?}", cmd);
        let root = match cmd.output() {
            Ok(output) => PathBuf::from(String::from_utf8_lossy(&output.stdout).trim()),
            Err(..) => self.verilator_root().unwrap_or_else(|| {
                // Set root to /verilator/path/to/exe/../
                let mut root = verilator_exe.clone();
                root.pop();
                root.pop();
                root
            }),
        };
        println!("verilator root: {:?}", root);

        let lib_name = format!("_{top_module}");
        // Generate .CPP from .V using verilator
        let mut cmd = Command::new(verilator_exe.clone());
        cmd.arg("--cc")
            .arg("-Mdir")
            .arg(&dst)
            .arg("--top-module")
            .arg(top_module)
            .arg("--lib-create")
            .arg(lib_name.clone());

        if self.coverage {
            cmd.arg("--coverage");
        }

        if self.trace {
            cmd.arg("--trace");
        }

        if self.optimized {
            cmd.arg("-O3");
        }

        for warn in &self.suppress_warnings {
            cmd.arg(format!("-Wno-{}", warn));
        }

        for dir in &self.module_directories {
            cmd.arg("-y");
            cmd.arg(dir);
        }

        for &(ref file, ref standard) in &self.files {
            if let Some(standard) = *standard {
                if let Some(ext) = file.extension() {
                    let flag = match standard {
                        Standard::Verilog1995 => &"+1364-1995ext",
                        Standard::Verilog2001 => &"+1364-2001ext",
                        Standard::Verilog2005 => &"+1364-2005ext",
                        Standard::SystemVerilog2005 => &"+1800-2005ext",
                        Standard::SystemVerilog2009 => &"+1800-2009ext",
                        Standard::SystemVerilog2012 => &"+1800-2012ext",
                    };
                    let flag = format!("{}+{}", flag, ext.to_string_lossy());
                    cmd.arg(flag);
                }
            }

            // verilator will generate relative path in .d files
            // which will cause make cannot find files.
            cmd.arg(fs::canonicalize(file).unwrap());
        }

        run(&mut cmd, "verilator");

        Command::new("make")
            .current_dir(dst.clone())
            .args(["-f", &format!("V{}.mk", top_module)])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        println!("cargo:rustc-link-search={}", dst.to_str().unwrap());
        println!("cargo:rustc-link-lib={lib_name}");
        dst
    }

    fn find_verilator_exe(&self) -> PathBuf {
        // Check ${VERILATOR_ROOT} first...
        if let Some(mut root) = self.verilator_root() {
            root.push("bin/verilator_bin");
            if root.is_file() {
                return root;
            }
        }

        // Otherwise, check PATH
        find_in_path("verilator_bin".as_ref())
    }

    fn verilator_root(&self) -> Option<PathBuf> {
        self.root
            .clone()
            .or_else(|| env::var_os("VERILATOR_ROOT").map(PathBuf::from))
    }
}

impl Default for Verilator {
    fn default() -> Verilator {
        Verilator {
            out_dir: None,
            root: None,
            files: Vec::new(),
            module_directories: Vec::new(),
            coverage: false,
            trace: false,
            optimized: false,
            suppress_warnings: Vec::new(),
        }
    }
}

fn find_in_path(path: &Path) -> PathBuf {
    env::split_paths(&env::var_os("PATH").unwrap_or_default())
        .map(|p| p.join(path))
        .find(|p| fs::metadata(p).is_ok())
        .unwrap_or_else(|| path.to_owned())
}

fn run(cmd: &mut Command, program: &str) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            fail(&format!(
                "failed to execute command: {}\nis `{}` not installed?",
                e, program
            ));
        }
        Err(e) => fail(&format!("failed to execute command: {}", e)),
    };
    if !status.success() {
        fail(&format!(
            "command did not execute successfully, got: {}",
            status
        ));
    }
}

fn getenv_unwrap(v: &str) -> String {
    match env::var(v) {
        Ok(s) => s,
        Err(..) => fail(&format!("environment variable `{}` not defined", v)),
    }
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}
