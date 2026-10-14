/*
** MSH - Munch Shell
** Copyright (c) 2026-present Tanvir
** SPDX-License-Identifier: Apache-2.0
*/

/* TTYLexDump - debug tool: tokenize input lines and dump tokens. */
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[path = "../../Source/TTY/Lexer.rs"]
mod Lexer;

fn DumpLine(line: &str) {
      let tokens = Lexer::Tokenize(line.trim());
      for (i, tk) in tokens.iter().enumerate() {
            println!("[{i}] {:?} val='{}'", tk.tk, tk.val);
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
