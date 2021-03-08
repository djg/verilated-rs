// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::{comments::extract_doc_comments, error::Diagnostic};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Result as SynResult},
    Token,
};

/// Provide a Diagnostic with the given span and message
macro_rules! err_span {
    ($span:expr, $($msg:tt)*) => (
        $crate::error::Diagnostic::spanned_error(&$span, format!($($msg)*))
    )
}

/// Immediately fail and return an Err, with the arguments passed to err_span!
#[macro_use]
macro_rules! bail_span {
    ($($t:tt)*) => (
        return Err(err_span!($($t)*).into())
    )
}

fn parse_ident(input: ParseStream) -> SynResult<Ident> {
    input.step(|cursor| match cursor.ident() {
        Some((ident, remaining)) => Ok((ident, remaining)),
        None => Err(cursor.error("expected an identifier")),
    })
}

#[derive(Default)]
pub struct VerilatedAttrs {
    pub attrs: Vec<VerilatedAttr>,
}

impl VerilatedAttrs {
    fn eval_end_step(&self) -> Option<Span> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                VerilatedAttr::EvalEndStep(span) => Some(*span),
                _ => None,
            })
            .next()
    }

    fn module(&self) -> Option<(&str, Span)> {
        self.attrs
            .iter()
            .filter_map(|a| match a {
                VerilatedAttr::Module(_, s, span) => Some((&s[..], *span)),
                _ => None,
            })
            .next()
    }
}

impl Parse for VerilatedAttrs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut attrs = VerilatedAttrs::default();
        if input.is_empty() {
            return Ok(attrs);
        }

        let opts = syn::punctuated::Punctuated::<_, syn::token::Comma>::parse_terminated(input)?;
        attrs.attrs = opts.into_iter().collect();
        Ok(attrs)
    }
}

/// The possible attributes in the `#[verilated]`.
pub enum VerilatedAttr {
    EvalEndStep(Span),
    Module(Span, String, Span),
}

impl Parse for VerilatedAttr {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let original = input.fork();
        let attr = parse_ident(input)?;
        let attr_span = attr.span();
        let attr_string = attr.to_string();

        if attr_string == "eval_end_step" {
            return Ok(VerilatedAttr::EvalEndStep(attr_span));
        }

        if attr_string == "module" {
            input.parse::<Token![=]>()?;
            let (val, span) = match input.parse::<syn::LitStr>() {
                Ok(str) => (str.value(), str.span()),
                Err(_) => {
                    let ident = parse_ident(input)?;
                    (ident.to_string(), ident.span())
                }
            };
            return Ok(VerilatedAttr::Module(attr_span, val, span));
        }

        Err(original.error("unknown attribute"))
    }
}

fn convert(item: &mut syn::ItemStruct, attrs: VerilatedAttrs) -> Result<Module, Diagnostic> {
    if !item.generics.params.is_empty() {
        bail_span!(
            item.generics,
            "structs with #[verilated] cannot have lifetime or type \
             parameters currently"
        );
    }
    if !item.fields.is_empty() {
        bail_span!(
            item.fields,
            "structs with #[verilated] cannot have fields currently"
        );
    }
    let verilog_name = attrs
        .module()
        .map(|s| s.0.to_owned())
        .unwrap_or_else(|| item.ident.to_string().to_lowercase());
    let comments: Vec<String> = extract_doc_comments(&item.attrs);
    let eval_end_step = attrs.eval_end_step().is_some();
    Ok(Module {
        rust_name: item.ident.clone(),
        verilog_name,
        comments,
        eval_end_step,
    })
}

/// Takes the parsed input from a `#[verilated]` macro and returns the generated bindings
pub fn expand(attr: TokenStream, input: TokenStream) -> Result<TokenStream, Diagnostic> {
    let mut item = syn::parse2::<syn::ItemStruct>(input)?;
    let opts = syn::parse2(attr)?;
    let module = convert(&mut item, opts)?;

    let mut tokens = proc_macro2::TokenStream::new();
    module.to_tokens(&mut tokens);
    Ok(tokens)
}

struct Module {
    /// The name of the struct is Rust code
    rust_name: Ident,
    /// The name of the module in Verilog code
    verilog_name: String,
    /// The doc comments on this struct, if provided
    comments: Vec<String>,
    /// Should eval_end_step be generated?
    eval_end_step: bool,
}

impl Module {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let binding_mod = Ident::new(&format!("v{}", self.verilog_name), Span::call_site());
        let binding_file = format!("/V{}.rs", self.verilog_name);
        let comments = &self.comments;
        let ffi_struct = Ident::new(&format!("V{}", self.verilog_name), Span::call_site());
        let ffi_constructor =
            Ident::new(&format!("V{0}_V{0}", self.verilog_name), Span::call_site());
        let rust_name = &self.rust_name;

        let eval_end_step = if !self.eval_end_step {
            quote! {
                unsafe {
                    self.0.eval_end_step();
                }
            }
        } else {
            quote! {}
        };

        (quote! {
            mod #binding_mod {
                #![allow(non_upper_case_globals)]
                #![allow(non_camel_case_types)]
                #![allow(non_snake_case)]

                include!(concat!(env!("OUT_DIR"), #binding_file));
            }

            #(#[ doc = #comments ])*
            pub struct #rust_name(#binding_mod::#ffi_struct);

            impl #rust_name {
                unsafe fn alloc(name: *const ::std::os::raw::c_char) -> *mut Self {
                    use ::std::alloc::{alloc, Layout};
                    let layout = Layout::new::<Self>();
                    let __tmp = alloc(layout) as *mut Self;
                    #binding_mod::#ffi_constructor(&mut (*__tmp).0, name);
                    __tmp as *mut Self
                }

                pub fn new(name: impl Into<Vec<u8>>) -> Result<Box<Self>, ::std::ffi::NulError>{
                    let name = ::std::ffi::CString::new(name)?;
                    let raw = unsafe { Self::alloc(name.as_ptr()) };
                    Ok(unsafe { Box::from_raw(raw) })
                }

                /// Trace signals in the model
                #[cfg(verilated = "trace-vcd")]
                fn trace_with_options(&mut self, vcd: &mut verilated::vcd::Vcd, levels: i32, options: i32) {
                    unsafe {
                        let vcd_c = vcd as *mut _ as *mut #binding_mod::VerilatedVcdC;
                        self.0.trace(vcd_c, levels, options)
                    }
                }

                /// Trace signals in the model
                #[cfg(verilated = "trace-vcd")]
                fn trace(&mut self, vcd: &mut verilated::vcd::Vcd, levels: i32) {
                    self.trace_with_options(vcd, levels, 0)
                }

                /// Trace signals in the model
                #[cfg(verilated = "trace-fst")]
                fn trace_with_options(&mut self, fst: &mut verilated::fst::Fst, levels: i32, options: i32) {
                    unsafe {
                        let fst_c = fst as *mut _ as *mut #binding_mod::VerilatedFstC;
                        self.0.trace(fst_c, levels, options)
                    }
                }

                /// Trace signals in the model
                #[cfg(verilated = "trace-fst")]
                fn trace(&mut self, fst: &mut verilated::fst::Fst, levels: i32) {
                    self.trace_with_options(fst, levels, 0)
                }

                /// Evaluate the model.  Application must call when inputs change.
                pub fn eval(&mut self) {
                    self.eval_step();
                    self.eval_end_step();
                }

                /// Evaluate when calling multiple units/models per time step.
                pub fn eval_step(&mut self) {
                    unsafe {
                        self.0.eval_step();
                    }
                }

                /// Evaluate at end of a timestep for tracing, when using eval_step().
                /// Application must call after all eval() and before time changes.
                pub fn eval_end_step(&mut self) {
                    #eval_end_step
                }

                /// Simulation complete, run final blocks.  Application must call on completion.
                pub fn finalize(&mut self) {
                    unsafe {
                        self.0.final_();
                    }
                }
            }

            impl Drop for #rust_name {
                fn drop(&mut self) {
                    unsafe {
                        self.0.destruct();
                    }
                }
            }

            impl std::ops::Deref for #rust_name {
                type Target = #binding_mod::#ffi_struct;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl std::ops::DerefMut for #rust_name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        })
        .to_tokens(tokens);
    }
}
