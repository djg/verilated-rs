// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::{comments::extract_doc_comments, error::Diagnostic};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Parser, Result as SynResult},
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

/// The possible attributes in the `#[verilated]`.
pub struct VerilatedAttr(Span, String, Span);

impl VerilatedAttr {
    pub fn opt_parse(input: ParseStream) -> SynResult<Option<Self>> {
        if input.is_empty() {
            return Ok(None);
        }

        let attr = Self::parse(input)?;
        Ok(Some(attr))
    }
}

impl Parse for VerilatedAttr {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let original = input.fork();
        let attr = parse_ident(input)?;
        let attr_span = attr.span();
        let attr_string = attr.to_string();

        if attr_string == "module" {
            input.parse::<Token![=]>()?;
            let (val, span) = match input.parse::<syn::LitStr>() {
                Ok(str) => (str.value(), str.span()),
                Err(_) => {
                    let ident = parse_ident(input)?;
                    (ident.to_string(), ident.span())
                }
            };
            return Ok(VerilatedAttr(attr_span, val, span));
        }

        Err(original.error("unknown attribute"))
    }
}

fn convert(item: &mut syn::ItemStruct, attr: Option<VerilatedAttr>) -> Result<Module, Diagnostic> {
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
    let verilog_name = attr
        .map(|s| s.1)
        .unwrap_or_else(|| item.ident.to_string().to_lowercase());
    let comments: Vec<String> = extract_doc_comments(&item.attrs);
    Ok(Module {
        rust_name: item.ident.clone(),
        verilog_name,
        comments,
    })
}

/// Takes the parsed input from a `#[verilated]` macro and returns the generated bindings
pub fn expand(attr: TokenStream, input: TokenStream) -> Result<TokenStream, Diagnostic> {
    let mut item = syn::parse2::<syn::ItemStruct>(input)?;
    let parser = VerilatedAttr::opt_parse;
    let opt = parser.parse2(attr)?;
    let module = convert(&mut item, opt)?;

    let mut tokens = proc_macro2::TokenStream::new();
    module.to_tokens(&mut tokens);
    Ok(tokens)
}

struct Module {
    /// The name of the struct is Rust code
    pub rust_name: Ident,
    /// The name of the module in Verilog code
    pub verilog_name: String,
    /// The doc comments on this struct, if provided
    pub comments: Vec<String>,
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

                /// Trace signals in the model; called by application code
                // void trace(VerilatedVcdC* tfp, int levels, int options = 0);

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
                    unsafe {
                        self.0.eval_end_step();
                    }
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
