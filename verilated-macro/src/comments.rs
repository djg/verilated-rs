// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Based upon code from wasm-bindgen
// Copyright (c) 2014 Alex Crichton

use proc_macro2::TokenTree;
use std::{char, str::Chars};

/// Extract the documentation comments from a Vec of attributes
pub fn extract_doc_comments(attrs: &[syn::Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter_map(|a| {
            // if the path segments include an ident of "doc" we know this
            // this is a doc comment
            if a.path.segments.iter().any(|s| s.ident == "doc") {
                Some(
                    // We want to filter out any Puncts so just grab the Literals
                    a.tokens.clone().into_iter().filter_map(|t| match t {
                        TokenTree::Literal(lit) => {
                            let quoted = lit.to_string();
                            Some(try_unescape(&quoted).unwrap_or(quoted))
                        }
                        _ => None,
                    }),
                )
            } else {
                None
            }
        })
        //Fold up the [[String]] iter we created into Vec<String>
        .fold(vec![], |mut acc, a| {
            acc.extend(a);
            acc
        })
}

// Unescapes a quoted string. char::escape_debug() was used to escape the text.
fn try_unescape(s: &str) -> Option<String> {
    if s.is_empty() {
        return Some(String::new());
    }
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    for i in 0.. {
        let c = match chars.next() {
            Some(c) => c,
            None => {
                if result.ends_with('"') {
                    result.pop();
                }
                return Some(result);
            }
        };
        if i == 0 && c == '"' {
            // ignore it
        } else if c == '\\' {
            let c = chars.next()?;
            match c {
                't' => result.push('\t'),
                'r' => result.push('\r'),
                'n' => result.push('\n'),
                '\\' | '\'' | '"' => result.push(c),
                'u' => {
                    if chars.next() != Some('{') {
                        return None;
                    }
                    let (c, next) = unescape_unicode(&mut chars)?;
                    result.push(c);
                    if next != '}' {
                        return None;
                    }
                }
                _ => return None,
            }
        } else {
            result.push(c);
        }
    }
    None
}

fn unescape_unicode(chars: &mut Chars) -> Option<(char, char)> {
    let mut value = 0;
    for i in 0..7 {
        let c = chars.next()?;
        let num = if ('0'..='9').contains(&c) {
            c as u32 - '0' as u32
        } else if ('a'..='f').contains(&c) {
            c as u32 - 'a' as u32 + 10
        } else if ('A'..='F').contains(&c) {
            c as u32 - 'A' as u32 + 10
        } else {
            if i == 0 {
                return None;
            }
            let decoded = char::from_u32(value)?;
            return Some((decoded, c));
        };
        if i >= 6 {
            return None;
        }
        value = (value << 4) | num;
    }
    None
}
