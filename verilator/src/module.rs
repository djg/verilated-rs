// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use fnv::FnvHashSet;

use std::{
    env,
    fs::File,
    io::prelude::*,
    io::BufWriter,
    path::{Path, PathBuf},
};

use syn::{
    self,
    punctuated::Punctuated,
    token::Comma,
    visit::{self, Visit},
    AttrStyle, Attribute, Fields, Generics, ItemStruct, Meta, NestedMeta, Visibility,
};

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => panic!("{} failed with {}", stringify!($e), e),
        }
    };
}

/// A builder used to generate verilator FFI shim.
pub struct ModuleGenerator {
    out_dir: Option<PathBuf>,
    target: Option<String>,
}

impl ModuleGenerator {
    /// Configures the output directory of the generated Rust and C code.
    ///
    /// Note that for Cargo builds this defaults to `$OUT_DIR` and it's not
    /// necessary to call.
    ///
    /// ```ignore
    /// use verilated::gen::ModuleGenerator;
    ///
    /// let mut cfg = ModuleGenerator::default();
    /// cfg.out_dir("path/to/output");
    /// ```
    pub fn out_dir<P>(&mut self, p: P) -> &mut ModuleGenerator
    where
        P: AsRef<Path>,
    {
        self.out_dir = Some(p.as_ref().to_owned());
        self
    }

    pub fn target(&mut self, target: &str) -> &mut ModuleGenerator {
        self.target = Some(target.to_string());
        self
    }

    /// Generate shim.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use verilated::gen::ModuleGenerator;
    ///
    /// let mut cfg = ModuleGenerator::new();
    /// cfg.generate("../path/to/lib.rs");
    /// ```
    pub fn generate<P>(&mut self, krate: P)
    where
        P: AsRef<Path>,
    {
        self._generate(krate.as_ref())
    }

    fn _generate(&mut self, krate: &Path) {
        self._generate_files(krate);
    }

    fn _generate_files(&mut self, krate: &Path) {
        let mut file = File::open(krate).expect("Unable to open file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Unable to read file");

        let ast = syn::parse_file(&content).expect("Unable to parse file");
        println!("{} items", ast.items.len());

        // Prep the code generator
        let out_dir = self
            .out_dir
            .clone()
            .unwrap_or_else(|| PathBuf::from(env::var_os("OUT_DIR").unwrap()));

        // Probe the crate to find all the structs of interest
        let mut structs = StructFinder {
            structs: FnvHashSet::default(),
        };
        visit::visit_file(&mut structs, &ast);

        let mut gen = Generator {
            out_dir: &out_dir,
            krate,
            found_module: false,
        };

        // Walk the crate, emitting modules for all modules found
        visit::visit_file(&mut gen, &ast);
    }
}

impl Default for ModuleGenerator {
    fn default() -> ModuleGenerator {
        ModuleGenerator {
            out_dir: None,
            target: None,
        }
    }
}

fn check_name(attr: &Attribute, name: &str) -> bool {
    if let Some(meta) = attr.interpret_meta() {
        meta.name() == name
    } else {
        false
    }
}

fn is_public(vis: &Visibility) -> bool {
    match vis {
        Visibility::Public(..) => true,
        _ => false,
    }
}

struct StructFinder {
    structs: FnvHashSet<String>,
}

impl<'ast> Visit<'ast> for StructFinder {
    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        let any_module = i.attrs.iter().any(|attr| {
            attr.style == AttrStyle::Outer && !attr.is_sugared_doc && check_name(attr, "module")
        });
        if any_module {
            self.structs.insert(i.ident.to_string());
        }
    }
}

struct Port {
    name: String,
    ty: String,
}

struct Ports {
    clock: Option<Port>,
    reset: Option<Port>,
    inputs: Vec<Port>,
    outputs: Vec<Port>,
    inouts: Vec<Port>,
}

struct Generator<'b> {
    out_dir: &'b PathBuf,
    krate: &'b Path,
    found_module: bool,
}

impl<'b> Generator<'b> {
    fn assert_no_generics(&self, generics: &Generics) {
        assert!(generics.params.is_empty());
        assert!(generics.where_clause.is_none());
    }

    fn gen_module(&mut self, rs_ty: &str, c_ty: &str, s: &ItemStruct) {
        let rs_file = self.out_dir.join(format!("{}.rs", rs_ty));
        let mut rs_out = BufWriter::new(t!(File::create(&rs_file)));

        let cpp_file = self.out_dir.join(format!("{}.cpp", c_ty));
        let mut cpp_out = BufWriter::new(t!(File::create(&cpp_file)));

        let ports = extract_ports(&s.fields);

        t!(writeln!(
            rs_out,
            r#"use std::path::Path;

mod ffi {{
    #[allow(non_camel_case_types)]
    pub enum {c_ty} {{}}

    extern {{
        pub fn {c_ty}_new() -> *mut {c_ty};
        pub fn {c_ty}_delete({c_ty}: *mut {c_ty});
        pub fn {c_ty}_eval({c_ty}: *mut {c_ty});
        pub fn {c_ty}_trace({c_ty}: *mut {c_ty}, vcd: *mut ::verilated::vcd::VcdC, levels: ::std::os::raw::c_int);
        pub fn {c_ty}_final({c_ty}: *mut {c_ty});"#,
            c_ty = c_ty
        ));

        t!(writeln!(
            cpp_out,
            r#"#include <V{c_ty}.h>

extern "C" {{
  // CONSTRUCTORS
  V{c_ty}*
  {c_ty}_new() {{
    V{c_ty}*ptr = new V{c_ty}();"#,
            c_ty = c_ty
        ));

        if let Some(clock) = &ports.clock {
            t!(writeln!(cpp_out, "    ptr->{clk} = 0;", clk = clock.name));
        }

        if let Some(reset) = &ports.reset {
            t!(writeln!(cpp_out, "    ptr->{rst} = 0;", rst = reset.name));
        }

        t!(writeln!(
            cpp_out,
            r#"    return ptr;
  }}

  void
  {c_ty}_delete(V{c_ty}* __ptr) {{
    delete __ptr;
  }}

  // API METHODS
  void
  {c_ty}_eval(V{c_ty}* __ptr) {{
    __ptr->eval();
  }}

  void
  {c_ty}_trace(V{c_ty}* __ptr, VerilatedVcdC* __tfp, int __levels) {{
    __ptr->trace(__tfp, __levels);
  }}

  void
  {c_ty}_final(V{c_ty}* __ptr) {{
    __ptr->final();
  }}
"#,
            c_ty = c_ty
        ));

        write_clock(&mut rs_out, &mut cpp_out, c_ty, &ports.clock);
        write_reset(&mut rs_out, &mut cpp_out, c_ty, &ports.reset);

        t!(writeln!(cpp_out, "  // PORTS"));
        write_inputs(&mut rs_out, &mut cpp_out, c_ty, &ports.inputs);
        write_outputs(&mut rs_out, &mut cpp_out, c_ty, &ports.outputs);
        write_inouts(&mut rs_out, &mut cpp_out, c_ty, &ports.inouts);

        t!(writeln!(
            rs_out,
            r#"    }}
}}
"#
        ));

        t!(writeln!(cpp_out, r#"}}"#));

        t!(writeln!(
            rs_out,
            r#"pub struct {rs_ty}(*mut ffi::{c_ty}, Option<::verilated::vcd::Vcd>);

impl Default for {rs_ty} {{
    fn default() -> Self {{
        let ptr = unsafe {{ ffi::{c_ty}_new() }};
        assert!(!ptr.is_null());
        {rs_ty}(ptr, None)
    }}
}}

impl Drop for {rs_ty} {{
    fn drop(&mut self) {{
        unsafe {{
            ffi::{c_ty}_delete(self.0);
        }}
    }}
}}

#[allow(dead_code, non_snake_case)]
impl {rs_ty} {{"#,
            c_ty = c_ty,
            rs_ty = rs_ty
        ));

        for input in &ports.inputs {
            t!(writeln!(
                rs_out,
                r#"    pub fn set_{input}(&mut self, v: {ty}) {{
        unsafe {{ ffi::{c_ty}_set_{input}(self.0, v); }}
    }}
"#,
                c_ty = c_ty,
                input = input.name,
                ty = &input.ty
            ));
        }

        for output in &ports.outputs {
            t!(writeln!(
                rs_out,
                r#"    pub fn {output}(&self) -> {ty} {{
        unsafe {{ ffi::{c_ty}_get_{output}(self.0) }}
    }}
"#,
                c_ty = c_ty,
                output = output.name,
                ty = &output.ty
            ));
        }

        for inout in &ports.inouts {
            t!(writeln!(
                rs_out,
                r#"    pub fn set_{inout}(&mut self, v: {ty}) {{
        unsafe {{ ffi::{c_ty}_set_{inout}(self.0, v); }}
    }}

    pub fn {inout}(&self) -> {ty} {{
        unsafe {{ ffi::{c_ty}_get_{inout}(self.0) }}
    }}
"#,
                c_ty = c_ty,
                inout = inout.name,
                ty = &inout.ty
            ));
        }

        t!(writeln!(
            rs_out,
            r#"
    pub fn eval(&mut self) {{
        unsafe {{
            ffi::{c_ty}_eval(self.0);
        }}
    }}

    pub fn finish(&mut self) {{
        unsafe {{
            ffi::{c_ty}_final(self.0);
        }}
    }}
"#,
            c_ty = c_ty,
        ));

        // Tracing API
        t!(writeln!(
            rs_out,
            r#"    pub fn open_trace<P: AsRef<Path>>(&mut self, path: P, levels: i32) -> std::io::Result<()> {{
        ::verilated::trace_ever_on(true);
        let mut vcd = ::verilated::vcd::Vcd::default();
        unsafe {{
            ffi::{c_ty}_trace(self.0, vcd.0, levels);
        }}
        vcd.open(path)?;
        self.1 = Some(vcd);
        Ok(())
    }}

    pub fn trace_at(&mut self, nanos: ::std::time::Duration) {{
        if let Some(ref mut vcd) = self.1 {{
            let timeui = nanos.as_secs() * 1_000_000_000 + u64::from(nanos.subsec_nanos());
            vcd.dump(timeui);
        }}
    }}
"#,
            c_ty = c_ty
        ));

        if let Some(clock) = ports.clock {
            t!(writeln!(
                rs_out,
                r#"    pub fn clock_toggle(&mut self) {{
        unsafe {{
            ffi::{c_ty}_{clk}_toggle(self.0);
        }}
    }}
"#,
                c_ty = c_ty,
                clk = clock.name
            ));
        } else {
            t!(writeln!(
                rs_out,
                r#"    pub fn clock_toggle(&mut self) {{
    unimplemented!();
}}
"#
            ));
        }

        if let Some(reset) = ports.reset {
            t!(writeln!(
                rs_out,
                r#"    pub fn reset_toggle(&mut self) {{
        unsafe {{
            ffi::{c_ty}_{rst}_toggle(self.0);
        }}
    }}
"#,
                c_ty = c_ty,
                rst = reset.name
            ));
        } else {
            t!(writeln!(
                rs_out,
                r#"    fn reset_up(&mut self) {{
    unimplemented!();
}}

fn reset_down(&mut self) {{
    unimplemented!();
}}
"#
            ));
        }

        t!(writeln!(rs_out, r#"}}"#));
    }
}

impl<'ast, 'b> Visit<'ast> for Generator<'b> {
    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        if !is_public(&i.vis) {
            return;
        }
        self.assert_no_generics(&i.generics);
        for attr in &i.attrs {
            let acc = find_module_attrs(attr);
            if !acc.is_empty() {
                let rs_ty = i.ident.to_string();
                let c_ty = &acc[0];
                self.gen_module(&rs_ty, &c_ty, i);

                if !self.found_module {
                    if let Some(path) = self.krate.to_str() {
                        println!("cargo:rerun-if-changed={}", path);
                    }
                }
                self.found_module = true;
            }
        }
    }
}

fn find_module_attrs(attr: &Attribute) -> Vec<String> {
    let mut acc = Vec::new();
    if let Some(meta) = attr.interpret_meta() {
        match meta {
            Meta::List(ref list) if meta.name() == "module" => {
                for item in &list.nested {
                    if let NestedMeta::Meta(Meta::Word(ref ident)) = item {
                        acc.push(ident.to_string())
                    }
                }
            }
            _ => {}
        }
    }
    acc
}

enum PortAttr {
    None,
    Clock,
    Reset,
    Input,
    Output,
    InOut,
}

type NestedMetaList = Punctuated<NestedMeta, Comma>;
fn list_contains_name(list: &NestedMetaList, name: &str) -> bool {
    list.iter().any(|meta| match meta {
        NestedMeta::Meta(Meta::Word(ref ident)) if ident == name => true,
        _ => false,
    })
}

fn find_port_attr(attrs: &[Attribute]) -> PortAttr {
    attrs.iter().fold(PortAttr::None, |pa, attr| {
        let meta = match attr.interpret_meta() {
            Some(meta) => meta,
            None => return pa,
        };
        match meta {
            syn::Meta::List(ref items) => {
                if items.ident != "port" {
                    return pa;
                }

                if items.nested.len() != 1 {
                    panic!("expected one argument");
                // PortAttr::None
                } else if list_contains_name(&items.nested, "clock") {
                    PortAttr::Clock
                } else if list_contains_name(&items.nested, "reset") {
                    PortAttr::Reset
                } else if list_contains_name(&items.nested, "input") {
                    PortAttr::Input
                } else if list_contains_name(&items.nested, "output") {
                    PortAttr::Output
                } else if list_contains_name(&items.nested, "inout") {
                    PortAttr::InOut
                } else {
                    panic!("invalid argument");
                    // PortAttr::None
                }
            }
            syn::Meta::Word(..) => {
                panic!("expected one argument");
                // PortAttr::None
            }
            _ => pa,
        }
    })
}

fn expr2width(e: &syn::Expr) -> usize {
    match e {
        syn::Expr::Lit(ref l) => match l.lit {
            syn::Lit::Int(ref a) => a.value() as usize,
            _ => panic!("unknown literal: {:?}", l),
        },
        _ => panic!("unknown expr: {:?}", e),
    }
}

fn ty2name(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(syn::TypePath { ref path, .. }) => {
            let last = path.segments.last().unwrap();
            if last.into_value().ident == "bool" {
                "u8".to_string()
            } else {
                panic!("only support bool");
            }
        }
        syn::Type::Array(arr) => {
            assert!(ty2name(&*arr.elem) == "u8");
            match expr2width(&arr.len) {
                0..=8 => "u8".to_string(),
                9..=16 => "u16".to_string(),
                17..=32 => "u32".to_string(),
                33..=64 => "u64".to_string(),
                _ => unreachable!(),
            }
        }
        _ => panic!("unknown ty {:?}", ty),
    }
}

fn rust2ffi(ty: &str) -> String {
    match ty {
        "u8" => "::std::os::raw::c_uchar".to_string(),
        "u16" => "::std::os::raw::c_ushort".to_string(),
        "u32" => "::std::os::raw::c_uint".to_string(),
        "u64" => "::std::os::raw::c_ulong".to_string(),
        s => s.to_string(),
    }
}

fn rust2ver(ty: &str) -> String {
    match ty {
        "u8" => "vluint8_t".to_string(),
        "u16" => "vluint16_t".to_string(),
        "u32" => "vluint32_t".to_string(),
        "u64" => "vluint64_t".to_string(),
        s => s.to_string(),
    }
}

fn extract_ports(fields: &Fields) -> Ports {
    let ports = Ports {
        clock: None,
        reset: None,
        inputs: Vec::new(),
        outputs: Vec::new(),
        inouts: Vec::new(),
    };

    match fields {
        Fields::Named(ref fields) => fields.named.iter().fold(ports, |mut ports, field| {
            let port = match field.ident {
                Some(name) => {
                    let name = name.to_string();
                    let ty = ty2name(&field.ty);
                    Port { name, ty }
                }
                None => panic!("no tuple structs in FFI"),
            };

            if is_public(&field.vis) {
                match find_port_attr(&field.attrs) {
                    PortAttr::Clock => {
                        if ports.clock.is_some() {
                            panic!("only one clock allowed in FFI");
                        }
                        ports.clock = Some(port);
                    }
                    PortAttr::Reset => {
                        if ports.reset.is_some() {
                            panic!("only one reset allowed in FFI");
                        }
                        ports.reset = Some(port);
                    }
                    PortAttr::Input => {
                        ports.inputs.push(port);
                    }
                    PortAttr::Output => {
                        ports.outputs.push(port);
                    }
                    PortAttr::InOut => {
                        ports.inouts.push(port);
                    }
                    _ => {}
                }
            }
            ports
        }),
        _ => panic!("no tuple structs in FFI"),
    }
}

fn write_clock<W>(rs_out: &mut W, cpp_out: &mut W, c_ty: &str, clock: &Option<Port>)
where
    W: Write,
{
    if let Some(ref clock) = clock {
        t!(writeln!(
            rs_out,
            r#"        pub fn {c_ty}_{clk}_toggle({c_ty}: *mut {c_ty});"#,
            c_ty = c_ty,
            clk = clock.name
        ));

        t!(writeln!(
            cpp_out,
            r#"  void
  {c_ty}_{clk}_toggle(V{c_ty}* __ptr) {{
    __ptr->{clk} = !__ptr->{clk};
  }}
"#,
            c_ty = c_ty,
            clk = clock.name
        ));
    }
}

fn write_reset<W>(rs_out: &mut W, cpp_out: &mut W, c_ty: &str, reset: &Option<Port>)
where
    W: Write,
{
    if let Some(ref reset) = reset {
        t!(writeln!(
            rs_out,
            r#"        pub fn {c_ty}_{rst}_toggle({c_ty}: *mut {c_ty});"#,
            c_ty = c_ty,
            rst = reset.name
        ));

        t!(writeln!(
            cpp_out,
            r#"  void
  {c_ty}_{rst}_toggle(V{c_ty}* __ptr) {{
    __ptr->{rst} = !__ptr->{rst};
  }}
"#,
            c_ty = c_ty,
            rst = reset.name
        ));
    }
}

fn write_inputs<W>(rs_out: &mut W, cpp_out: &mut W, c_ty: &str, inputs: &[Port])
where
    W: Write,
{
    for input in inputs {
        t!(writeln!(
            rs_out,
            r#"        pub fn {c_ty}_set_{input}({c_ty}: *mut {c_ty}, v: {ffi_ty});"#,
            c_ty = c_ty,
            input = input.name,
            ffi_ty = rust2ffi(&input.ty)
        ));

        t!(writeln!(
            cpp_out,
            r#"  void
  {c_ty}_set_{input}(V{c_ty}* __ptr, {v_ty} __v) {{
    __ptr->{input} = __v;
  }}
"#,
            c_ty = c_ty,
            input = input.name,
            v_ty = rust2ver(&input.ty)
        ));
    }
}

fn write_outputs<W>(rs_out: &mut W, cpp_out: &mut W, c_ty: &str, outputs: &[Port])
where
    W: Write,
{
    for output in outputs {
        t!(writeln!(
            rs_out,
            r#"        pub fn {c_ty}_get_{output}({c_ty}: *mut {c_ty}) -> {ffi_ty};"#,
            c_ty = c_ty,
            output = output.name,
            ffi_ty = rust2ffi(&output.ty)
        ));

        t!(writeln!(
            cpp_out,
            r#"  {v_ty}
  {c_ty}_get_{output}(V{c_ty}* __ptr) {{
    return __ptr->{output};
  }}
"#,
            c_ty = c_ty,
            output = output.name,
            v_ty = rust2ver(&output.ty)
        ));
    }
}

fn write_inouts<W>(rs_out: &mut W, cpp_out: &mut W, c_ty: &str, inouts: &[Port])
where
    W: Write,
{
    for inout in inouts {
        t!(writeln!(
            rs_out,
            r#"        pub fn {c_ty}_set_{inout}({c_ty}: *mut {c_ty}, v: {ffi_ty});
        pub fn {c_ty}_get_{inout}({c_ty}: *mut {c_ty}) -> {ffi_ty};"#,
            c_ty = c_ty,
            inout = inout.name,
            ffi_ty = rust2ffi(&inout.ty)
        ));

        t!(writeln!(
            cpp_out,
            r#"  void
  {c_ty}_set_{inout}(V{c_ty}* __ptr, {v_ty} __v) {{
    __ptr->{inout} = __v;
  }}

  {v_ty}
  {c_ty}_get_{inout}(V{c_ty}* __ptr) {{
    return __ptr->{inout};
  }}
"#,
            c_ty = c_ty,
            inout = inout.name,
            v_ty = rust2ver(&inout.ty)
        ));
    }
}
