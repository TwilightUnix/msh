/*
** MSH - Munch Shell
** Copyright (c) 2026-present TwilightUnix and contributors
** SPDX-License-Identifier: Apache-2.0
*/

/* TTYParseDump - debug tool: tokenize + parse input lines and dump the AST. */

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::needless_return)]

#[path = "../../Source/TTY/Error.rs"]
pub mod Error;

#[path = "../../Source/TTY/Lexer.rs"]
pub mod Lexer;

#[path = "../../Source/TTY/Parser.rs"]
pub mod Parser;

/* Source files reference each other as `crate::TTY::...`, so mirror that here. */
pub mod TTY {
      pub use crate::{Error, Lexer, Parser};
}

fn DumpLine(line: &str) {
      let tokens = TTY::Lexer::Tokenize(line.trim());
      match TTY::Parser::ParseList(&tokens) {
            Ok(list) => println!("{list:#?}"),
            Err(err) => println!("{err}"),
      }
}

fn main() {
      let args: Vec<String> = std::env::args().skip(1).collect();
      if !args.is_empty() {
            DumpLine(&args.join(" "));
            return;
      }

      use std::io::BufRead;
      let stdin = std::io::stdin();
      for line in stdin.lock().lines() {
            match line {
                  Ok(text) => DumpLine(&text),
                  Err(err) => {
                        eprintln!("msh: read error: {err}");
                        break;
                  }
            }
      }
}
