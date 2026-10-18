/*
** MSH - Munch Shell
** Copyright (c) 2026-present TwilightUnix and contributors
** SPDX-License-Identifier: Apache-2.0
*/

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::env::{self};
use std::io::Write;

mod Cli;
mod TTY;

fn RunCommandLine() {
      loop {
            let mut input = String::new();
            print!("msh $ ");

            std::io::stdout().flush().expect("Failed to read line");

            std::io::stdin()
                  .read_line(&mut input)
                  .expect("Failed to read line");

            let tokens = crate::TTY::Lexer::Tokenize(input.trim());
            let list = crate::TTY::Parser::ParseList(&tokens);
      }
}

fn main() {
      let Args: Vec<String> = env::args().collect();
      if !(Args.len() > 1) {
            RunCommandLine();
      }

      crate::Cli::ParseOption(Args);
}
