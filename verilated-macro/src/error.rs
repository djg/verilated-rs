// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Based upon code from wasm-bindgen
// Copyright (c) 2014 Alex Crichton

use proc_macro2::*;
use quote::{ToTokens, TokenStreamExt};
use syn::parse::Error;

/// A struct representing a diagnostic to emit to the end-user as an error.
#[derive(Debug)]
pub struct Diagnostic {
    inner: Repr,
}

#[derive(Debug)]
enum Repr {
    Single {
        text: String,
        span: Option<(Span, Span)>,
    },
    SynError(Error),
}

impl Diagnostic {
    /// Generate a `Diagnostic` from the span of any tokenizable object and a message
    pub fn spanned_error<T: Into<String>>(node: &dyn ToTokens, text: T) -> Diagnostic {
        Diagnostic {
            inner: Repr::Single {
                text: text.into(),
                span: extract_spans(node),
            },
        }
    }
}

impl From<Error> for Diagnostic {
    fn from(err: Error) -> Diagnostic {
        Diagnostic {
            inner: Repr::SynError(err),
        }
    }
}

fn extract_spans(node: &dyn ToTokens) -> Option<(Span, Span)> {
    let mut t = TokenStream::new();
    node.to_tokens(&mut t);
    let mut tokens = t.into_iter();
    let start = tokens.next().map(|t| t.span());
    let end = tokens.last().map(|t| t.span());
    start.map(|start| (start, end.unwrap_or(start)))
}

impl ToTokens for Diagnostic {
    fn to_tokens(&self, dst: &mut TokenStream) {
        match &self.inner {
            Repr::Single { text, span } => {
                let cs2 = (Span::call_site(), Span::call_site());
                let (start, end) = span.unwrap_or(cs2);
                dst.append(Ident::new("compile_error", start));
                dst.append(Punct::new('!', Spacing::Alone));
                let mut message = TokenStream::new();
                message.append(Literal::string(text));
                let mut group = Group::new(Delimiter::Brace, message);
                group.set_span(end);
                dst.append(group);
            }
            Repr::SynError(err) => {
                err.to_compile_error().to_tokens(dst);
            }
        }
    }
}
