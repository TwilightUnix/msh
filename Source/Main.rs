/*
** MSH - Munch Shell
** Copyright (c) 2026-present TwilightUnix and contributors
** SPDX-License-Identifier: Apache-2.0
*/

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::needless_return)]

use msh::Cli;
use msh::TTY;
use std::env::{self};
use std::io::Write;

fn RunCommandLine() {
      loop {
            let mut input = String::new();
            print!("msh $ ");

            std::io::stdout().flush().expect("Failed to read line");

            std::io::stdin()
                  .read_line(&mut input)
                  .expect("Failed to read line");

            if input.is_empty() {
                  break; // Ctrl-D (EOF)
            }

            let tokens = TTY::Lexer::Tokenize(input.trim());
            if let Err(err) = TTY::Parser::ParseList(&tokens) {
                  println!("{err}");
            }
      }
}

fn main() {
      let Args: Vec<String> = env::args().collect();
      if !(Args.len() > 1) {
            RunCommandLine();
      }

      Cli::ParseOption(Args);
}
