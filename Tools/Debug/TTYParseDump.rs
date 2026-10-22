/*
** MSH - Munch Shell
** Copyright (c) 2026-present TwilightUnix and contributors
** SPDX-License-Identifier: Apache-2.0
*/

/* TTYParseDump - debug tool: tokenize + parse input lines and dump the AST. */

#![allow(non_snake_case)]
#![allow(clippy::needless_return)]

use msh::TTY::Lexer;
use msh::TTY::Parser;

fn DumpLine(line: &str) {
      let tokens = Lexer::Tokenize(line.trim());
      match Parser::ParseList(&tokens) {
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
