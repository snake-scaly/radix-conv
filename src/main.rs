// Copyright (c) 2015 Sergey "SnakE" Gromov
//
// See the file license.txt for copying permission.

//! # Radix Conversion Utility

extern crate num;

mod table;
mod convtable;

use std::{env, path};
use num::BigInt;
use convtable::ConvTable;

use std::error::Error;
use num::traits::Num;

fn main() {
    let mut table = ConvTable::new();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        usage(path::Path::new(&args[0]).file_name().unwrap().to_str().unwrap());
        return;
    }

    for arg in args.into_iter().skip(1) {
        let arg = arg.trim();
        let (v, radix) = if let Some(s) = strip_prefix(&arg, "0x") {
                (s, 16)
            } else if let Some(s) = strip_prefix(&arg, "0b") {
                (s, 2)
            } else if let Some(s) = strip_prefix(&arg, "0o") {
                (s, 8)
            } else {
                (&*arg, 10)
            };

        match BigInt::from_str_radix(&v, radix) {
            Ok(x) => table.push_result(&arg, &x),
            Err(e) => table.push_error(&arg, e.description()),
        };
    }

    table.print();
}

/// Return input string without prefix if prefix matches.
fn strip_prefix<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    if s.starts_with(prefix) {
        Some(&s[prefix.len()..])
    } else {
        None
    }
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn usage(tool: &str) {
    println!("\
Display numbers in multiple radii
(c) 2015 Sergey \"SnakE\" Gromov
Version {}

Usage: {} num [num ...]
  num      decimal, hex, octal, or binary number
  decimal  start with a digit
  hex      start with `0x`
  octal    start with `0o`
  binary   start with `0b`", VERSION, tool);
}
