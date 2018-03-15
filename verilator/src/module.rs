use fnv::{FnvHashMap, FnvHashSet};

use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use syntax::ast::{self, AttrStyle, Attribute, MetaItemKind, Name};
use syntax::attr;
use syntax::codemap::FilePathMapping;
use syntax::config::StripUnconfigured;
use syntax::errors::Handler;
use syntax::ext::base::{Determinacy, ExtCtxt, MacroKind, Resolver, SyntaxExtension};
use syntax::ext::expand::{Expansion, ExpansionConfig, Invocation, InvocationKind};
use syntax::ext::hygiene::Mark;
use syntax::ext::tt::macro_rules;
use syntax::feature_gate::Features;
use syntax::fold::{self, Folder};
use syntax::parse::{self, ParseSess};
use syntax::ptr::P;
use syntax::util::small_vector::SmallVector;
use syntax::visit::{self, Visitor};

macro_rules! t {
    ($e:expr) => (match $e {
        Ok(e) => e,
        Err(e) => panic!("{} failed with {}", stringify!($e), e),
    })
}

/// A builder used to generate verilator FFI shim.
pub struct ModuleGenerator {
    out_dir: Option<PathBuf>,
    target: Option<String>,
}

impl ModuleGenerator {
    /// Creates a new blank shim generator
    pub fn new() -> ModuleGenerator {
        ModuleGenerator {
            out_dir: None,
            target: None,
        }
    }

    /// Configures the output directory of the generated Rust and C code.
    ///
    /// Note that for Cargo builds this defaults to `$OUT_DIR` and it's not
    /// necessary to call.
    ///
    /// ```no_run
    /// use verilated::gen::ModuleGenerator;
    ///
    /// let mut cfg = ModuleGenerator::new();
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
    /// ```no_run
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
        // Prep the code generator
        let out_dir = self.out_dir
            .clone()
            .unwrap_or_else(|| PathBuf::from(env::var_os("OUT_DIR").unwrap()));

        let target = self.target
            .clone()
            .unwrap_or_else(|| env::var("TARGET").unwrap());

        let mut sess = ParseSess::new(FilePathMapping::empty());
        for (k, v) in default_cfg(&target).into_iter() {
            let s = |s: &str| ast::Name::intern(s);
            sess.config.insert((s(&k), v.as_ref().map(|n| s(n))));
        }

        // parse the crate `krate`.
        let krate = parse::parse_crate_from_file(krate, &sess).ok().unwrap();

        // Remove things like functions, impls, traits, etc, that we're
        // not looking for.
        let krate = Uninterested.fold_crate(krate);

        // expand macros
        let features = Features::new();
        let mut ecfg = ExpansionConfig {
            features: Some(&features),
            ..ExpansionConfig::default("crate_name".to_string())
        };
        ecfg.recursion_limit = 128;

        let mut resolver = MyResolver {
            parse_sess: &sess,
            map: FnvHashMap::default(),
            id: 1_000_000_000,
        };
        let mut ecx = ExtCtxt::new(&sess, ecfg, &mut resolver);
        let krate = ecx.monotonic_expander().expand_crate(krate);

        // Strip the crate down to just what's configured for our target
        let krate = StripUnconfigured {
            should_test: false,
            sess: &sess,
            features: None,
        }.fold_crate(krate);

        // Probe the crate to find all the structs of interest
        let mut structs = StructFinder {
            structs: FnvHashSet::default(),
        };
        visit::walk_crate(&mut structs, &krate);

        let mut gen = Generator {
            sh: &sess.span_diagnostic,
            files: FnvHashSet::default(),
            sess: &sess,
            out_dir: &out_dir,
        };

        // Walk the crate, emitting modules for all modules found
        visit::walk_crate(&mut gen, &krate);
    }
}

fn default_cfg(target: &str) -> Vec<(String, Option<String>)> {
    let mut ret = Vec::new();
    let (arch, width) = if target.starts_with("x86_64") {
        if target.ends_with("x32") {
            ("x86_64", "32")
        } else {
            ("x86_64", "64")
        }
    } else if target.starts_with("i386") || target.starts_with("i586") || target.starts_with("i686")
    {
        ("x86", "32")
    } else if target.starts_with("arm") {
        ("arm", "32")
    } else if target.starts_with("aarch64") {
        ("aarch64", "64")
    } else if target.starts_with("mips64") {
        ("mips64", "64")
    } else if target.starts_with("mips") {
        ("mips", "32")
    } else if target.starts_with("powerpc64") {
        ("powerpc64", "64")
    } else if target.starts_with("powerpc") {
        ("powerpc", "32")
    } else if target.starts_with("s390x") {
        ("s390x", "64")
    } else if target.starts_with("sparc64") {
        ("sparc64", "64")
    } else if target.starts_with("asmjs") {
        ("asmjs", "32")
    } else if target.starts_with("wasm32") {
        ("wasm32", "32")
    } else {
        panic!("unknown arch/pointer width: {}", target)
    };
    let (os, family, env) = if target.contains("unknown-linux-gnu") {
        ("linux", "unix", "gnu")
    } else if target.contains("unknown-linux-musl") {
        ("linux", "unix", "musl")
    } else if target.contains("unknown-linux-uclibc") {
        ("linux", "unix", "uclibc")
    } else if target.contains("apple-darwin") {
        ("macos", "unix", "")
    } else if target.contains("apple-ios") {
        ("ios", "unix", "")
    } else if target.contains("windows-msvc") {
        ("windows", "windows", "msvc")
    } else if target.contains("windows-gnu") {
        ("windows", "windows", "gnu")
    } else if target.contains("android") {
        ("android", "unix", "")
    } else if target.contains("unknown-freebsd") {
        ("freebsd", "unix", "")
    } else if target.contains("netbsd") {
        ("netbsd", "unix", "")
    } else if target.contains("openbsd") {
        ("openbsd", "unix", "")
    } else if target.contains("dragonfly") {
        ("dragonfly", "unix", "")
    } else if target.contains("solaris") {
        ("solaris", "unix", "")
    } else if target.contains("emscripten") {
        ("emscripten", "unix", "")
    } else {
        panic!("unknown os/family width: {}", target)
    };

    // TODO: endianness
    ret.push((family.to_string(), None));
    ret.push(("target_os".to_string(), Some(os.to_string())));
    ret.push(("target_family".to_string(), Some(family.to_string())));
    ret.push(("target_arch".to_string(), Some(arch.to_string())));
    ret.push(("target_pointer_width".to_string(), Some(width.to_string())));
    ret.push(("target_env".to_string(), Some(env.to_string())));

    return ret;
}

struct Uninterested;

impl Folder for Uninterested {
    fn fold_item(&mut self, item: P<ast::Item>) -> SmallVector<P<ast::Item>> {
        match item.node {
            ast::ItemKind::Mod(..)
            | ast::ItemKind::ForeignMod(..)
            | ast::ItemKind::Ty(..)
            | ast::ItemKind::Struct(..)
            | ast::ItemKind::Mac(..)
            | ast::ItemKind::MacroDef(..)
            | ast::ItemKind::Use(..)
            | ast::ItemKind::ExternCrate(..)
            | ast::ItemKind::Const(..) => return fold::noop_fold_item(item, self),

            ast::ItemKind::Static(..)
            | ast::ItemKind::Enum(..)
            | ast::ItemKind::Fn(..)
            | ast::ItemKind::GlobalAsm(..)
            | ast::ItemKind::Trait(..)
            | ast::ItemKind::DefaultImpl(..)
            | ast::ItemKind::Impl(..)
            | ast::ItemKind::Union(..) => return Default::default(),
        }
    }

    fn fold_mac(&mut self, mac: ast::Mac) -> ast::Mac {
        fold::noop_fold_mac(mac, self)
    }
}

struct MyResolver<'a> {
    parse_sess: &'a ParseSess,
    id: usize,
    map: FnvHashMap<Name, Rc<SyntaxExtension>>,
}

impl<'a> Resolver for MyResolver<'a> {
    fn next_node_id(&mut self) -> ast::NodeId {
        self.id += 1;
        ast::NodeId::new(self.id)
    }

    fn get_module_scope(&mut self, _id: ast::NodeId) -> Mark {
        Mark::root()
    }

    fn eliminate_crate_var(&mut self, item: P<ast::Item>) -> P<ast::Item> {
        item
    }

    fn is_whitelisted_legacy_custom_derive(&self, _name: Name) -> bool {
        false
    }

    fn visit_expansion(&mut self, _invoc: Mark, expansion: &Expansion, _derives: &[Mark]) {
        match *expansion {
            Expansion::Items(ref items) => {
                let features = RefCell::new(Features::new());
                for item in items.iter() {
                    MyVisitor {
                        parse_sess: self.parse_sess,
                        features: &features,
                        map: &mut self.map,
                    }.visit_item(item);
                }
            }
            _ => {}
        }
    }

    fn add_builtin(&mut self, _ident: ast::Ident, _ext: Rc<SyntaxExtension>) {}

    fn resolve_imports(&mut self) {}

    fn find_legacy_attr_invoc(&mut self, _attrs: &mut Vec<Attribute>) -> Option<Attribute> {
        //        attrs.retain(|a| !a.check_name("derive"));
        None
    }

    fn resolve_invoc(
        &mut self,
        invoc: &mut Invocation,
        _scope: Mark,
        _force: bool,
    ) -> Result<Option<Rc<SyntaxExtension>>, Determinacy> {
        match invoc.kind {
            InvocationKind::Bang { ref mac, .. } => {
                if mac.node.path.segments.len() != 1 {
                    return Ok(None);
                }
                let seg = &mac.node.path.segments[0];
                if seg.parameters.is_some() {
                    return Ok(None);
                }
                return Ok(self.map.get(&seg.identifier.name).cloned());
            }
            _ => {}
        }
        Err(Determinacy::Determined)
    }

    fn resolve_macro(
        &mut self,
        _scope: Mark,
        _path: &ast::Path,
        _kind: MacroKind,
        _force: bool,
    ) -> Result<Rc<SyntaxExtension>, Determinacy> {
        Err(Determinacy::Determined)
    }

    fn check_unused_macros(&self) {}
}

struct MyVisitor<'b> {
    parse_sess: &'b ParseSess,
    features: &'b RefCell<Features>,
    map: &'b mut FnvHashMap<Name, Rc<SyntaxExtension>>,
}

impl<'a, 'b> Visitor<'a> for MyVisitor<'b> {
    fn visit_item(&mut self, item: &'a ast::Item) {
        match item.node {
            ast::ItemKind::MacroDef(..) => {
                self.map.insert(
                    item.ident.name,
                    Rc::new(macro_rules::compile(self.parse_sess, self.features, item)),
                );
            }
            _ => {}
        }
        visit::walk_item(self, item);
    }

    fn visit_mac(&mut self, mac: &'a ast::Mac) {
        drop(mac);
    }
}

struct StructFinder {
    structs: FnvHashSet<String>,
}

impl<'a> Visitor<'a> for StructFinder {
    fn visit_item(&mut self, i: &'a ast::Item) {
        match i.node {
            ast::ItemKind::Struct(..) => {
                if i.attrs.iter().any(|ref attr| {
                    attr.style == AttrStyle::Outer && attr.is_sugared_doc == false
                        && attr.check_name("module")
                }) {
                    self.structs.insert(i.ident.to_string());
                }
            }
            _ => {}
        }
        visit::walk_item(self, i)
    }
    fn visit_mac(&mut self, _mac: &'a ast::Mac) {}
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
    sh: &'b Handler,
    files: FnvHashSet<String>,
    sess: &'b ParseSess,
    out_dir: &'b PathBuf,
}

impl<'b> Generator<'b> {
    fn assert_no_generics(&self, generics: &ast::Generics) {
        assert!(generics.lifetimes.is_empty());
        assert!(generics.ty_params.is_empty());
        assert!(generics.where_clause.predicates.is_empty());
    }

    fn gen_module(&mut self, rs_ty: &str, c_ty: &str, s: &ast::VariantData) {
        let rs_file = self.out_dir.join(format!("{}.rs", rs_ty));
        let mut rs_out = BufWriter::new(t!(File::create(&rs_file)));

        let cpp_file = self.out_dir.join(format!("{}.cpp", c_ty));
        let mut cpp_out = BufWriter::new(t!(File::create(&cpp_file)));

        let ports = Ports {
            clock: None,
            reset: None,
            inputs: Vec::new(),
            outputs: Vec::new(),
            inouts: Vec::new(),
        };

        let ports = s.fields().iter().fold(ports, |mut ports, field| {
            let port = match field.ident {
                Some(name) => {
                    let name = name.to_string();
                    let ty = ty2name(&*field.ty);
                    Port { name, ty }
                }
                None => panic!("no tuple structs in FFI"),
            };

            if field.vis == ast::Visibility::Public {
                match find_port_attr(self.sh, &field.attrs) {
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
        });

        t!(writeln!(
            rs_out,
            r#"mod ffi {{
    #[allow(non_camel_case_types)]
    pub enum {c_ty} {{}}

    extern {{
        pub fn {c_ty}_new() -> *mut {c_ty};
        pub fn {c_ty}_delete({c_ty}: *mut {c_ty});
        pub fn {c_ty}_eval({c_ty}: *mut {c_ty});
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
    return new V{c_ty}();
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
  {c_ty}_final(V{c_ty}* __ptr) {{
    __ptr->final();
  }}
"#,
            c_ty = c_ty
        ));

        if let Some(ref clock) = ports.clock {
            t!(writeln!(
                rs_out,
                r#"        pub fn {c_ty}_{clk}({c_ty}: *mut {c_ty}, v: u8);"#,
                c_ty = c_ty,
                clk = clock.name
            ));

            t!(writeln!(
                cpp_out,
                r#"  void
  {c_ty}_{clk}(V{c_ty}* __ptr, vluint8_t __v) {{
    __ptr->{clk} = __v;
  }}
"#,
                c_ty = c_ty,
                clk = clock.name
            ));
        }

        if let Some(ref reset) = ports.reset {
            t!(writeln!(
                rs_out,
                r#"        pub fn {c_ty}_{rst}({c_ty}: *mut {c_ty}, v: u8);"#,
                c_ty = c_ty,
                rst = reset.name
            ));

            t!(writeln!(
                cpp_out,
                r#"  void
  {c_ty}_{rst}(V{c_ty}* __ptr, vluint8_t __v) {{
    __ptr->{rst} = __v;
  }}
"#,
                c_ty = c_ty,
                rst = reset.name
            ));
        }

        t!(writeln!(cpp_out, "  // PORTS"));

        for input in &ports.inputs {
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

        for output in &ports.outputs {
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

        for inout in &ports.inouts {
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

        t!(writeln!(
            rs_out,
            r#"    }}
}}
"#
        ));

        t!(writeln!(cpp_out, r#"}}"#));

        t!(writeln!(
            rs_out,
            r#"pub struct {rs_ty}(*mut ffi::{c_ty});

impl Default for {rs_ty} {{
    fn default() -> Self {{
        let ptr = unsafe {{ ffi::{c_ty}_new() }};
        assert!(!ptr.is_null());
        {rs_ty}(ptr)
    }}
}}

impl Drop for {rs_ty} {{
    fn drop(&mut self) {{
        unsafe {{
            ffi::{c_ty}_delete(self.0);
        }}
    }}
}}

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
            r#"}}
"#
        ));

        t!(writeln!(
            rs_out,
            r#"impl ::verilated::test_bench::Module for {rs_ty} {{

    fn eval(&mut self) {{
        unsafe {{
            ffi::{c_ty}_eval(self.0);
        }}
    }}

    fn finish(&mut self) {{
        unsafe {{
            ffi::{c_ty}_final(self.0);
        }}
    }}
"#,
            c_ty = c_ty,
            rs_ty = rs_ty,
        ));

        if let Some(clock) = ports.clock {
            t!(writeln!(
                rs_out,
                r#"    fn clock_up(&mut self) {{
        unsafe {{
            ffi::{c_ty}_{clk}(self.0, 1);
        }}
    }}

    fn clock_down(&mut self) {{
        unsafe {{
            ffi::{c_ty}_{clk}(self.0, 0);
        }}
    }}
"#,
                c_ty = c_ty,
                clk = clock.name
            ));
        } else {
            t!(writeln!(
                rs_out,
                r#"    fn clock_up(&mut self) {{
    unimplemented!();
}}

fn clock_down(&mut self) {{
    unimplemented!();
}}
"#
            ));
        }

        if let Some(reset) = ports.reset {
            t!(writeln!(
                rs_out,
                r#"    fn reset_up(&mut self) {{
        unsafe {{
            ffi::{c_ty}_{rst}(self.0, 1);
        }}
    }}

    fn reset_down(&mut self) {{
        unsafe {{
            ffi::{c_ty}_{rst}(self.0, 0);
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

impl<'a, 'b> Visitor<'a> for Generator<'b> {
    fn visit_item(&mut self, i: &'a ast::Item) {
        let public = i.vis == ast::Visibility::Public;
        match i.node {
            ast::ItemKind::Struct(ref s, ref generics) if public => {
                self.assert_no_generics(generics);
                for attr in &i.attrs {
                    let acc = find_module_attrs(self.sh, attr);
                    if acc.len() > 0 {
                        let rs_ty = i.ident.to_string();
                        let c_ty = &acc[0];
                        self.gen_module(&rs_ty, &c_ty, s);
                    }
                }
            }

            _ => {}
        }
        let file = self.sess.codemap().span_to_filename(i.span);
        if self.files.insert(file.clone()) {
            println!("cargo:rerun-if-changed={}", file);
        }
        visit::walk_item(self, i);
    }
}

fn find_module_attrs(diagnostic: &Handler, attr: &Attribute) -> Vec<String> {
    let mut acc = Vec::new();
    if attr.path == "module" {
        if let Some(items) = attr.meta_item_list() {
            attr::mark_used(attr);
            for item in items {
                if !item.is_meta_item() {
                    diagnostic.span_err_with_code(item.span, "unsupported literal", "E0565");
                    continue;
                }

                if let Some(mi) = item.word() {
                    acc.push(mi.name().as_str().to_string())
                }
            }
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

fn find_port_attr(diagnostics: &Handler, attrs: &[Attribute]) -> PortAttr {
    attrs.iter().fold(PortAttr::None, |pa, attr| {
        if attr.path != "port" {
            return pa;
        }
        let meta = match attr.meta() {
            Some(meta) => meta.node,
            None => return pa,
        };
        match meta {
            MetaItemKind::List(ref items) => {
                attr::mark_used(attr);
                if items.len() != 1 {
                    diagnostics.span_err_with_code(attr.span, "expected one argument", "E0534");
                    PortAttr::None
                } else if attr::list_contains_name(&items[..], "clock") {
                    PortAttr::Clock
                } else if attr::list_contains_name(&items[..], "reset") {
                    PortAttr::Reset
                } else if attr::list_contains_name(&items[..], "input") {
                    PortAttr::Input
                } else if attr::list_contains_name(&items[..], "output") {
                    PortAttr::Output
                } else if attr::list_contains_name(&items[..], "inout") {
                    PortAttr::InOut
                } else {
                    diagnostics.span_err_with_code(items[0].span, "invalid argument", "E0535");
                    PortAttr::None
                }
            }
            MetaItemKind::Word => {
                diagnostics.span_err_with_code(attr.span, "expected one argument", "E0534");
                PortAttr::None
            }
            _ => pa,
        }
    })
}

fn expr2width(e: &ast::Expr) -> usize {
    match e.node {
        ast::ExprKind::Lit(ref l) => match l.node {
            ast::LitKind::Int(a, _) => {
                if a.as_built_in() > 64 {
                    panic!("maximum port width of 64 supported");
                }
                a.low64() as usize
            }
            _ => panic!("unknown literal: {:?}", l),
        },
        _ => panic!("unknown expr: {:?}", e),
    }
}

fn ty2name(ty: &ast::Ty) -> String {
    match ty.node {
        ast::TyKind::Path(_, ref path) => {
            let last = path.segments.last().unwrap();
            if last.identifier.to_string() == "bool" {
                "u8".to_string()
            } else {
                panic!("only support bool");
            }
        }
        ast::TyKind::Array(ref t, ref e) => {
            assert!(ty2name(t) == "u8");
            match expr2width(e) {
                0...8 => "u8".to_string(),
                9...16 => "u16".to_string(),
                17...32 => "u32".to_string(),
                33...64 => "u64".to_string(),
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
