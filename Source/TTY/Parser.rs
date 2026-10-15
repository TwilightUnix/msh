/*
** MSH - Munch Shell
** Copyright (c) 2026-present Tanvir
** SPDX-License-Identifier: Apache-2.0
*/

/* Parser.rs - tty parser for msh */

use crate::TTY::Lexer::Token;

pub enum Separator {
      SP_Semicolon,  // ;
      SP_Background, // &
      SP_And,        // &&
      SP_Or,         // ||
}

pub struct SimpleCommand<'a> {
      pub args: Vec<Token<'a>>,
      pub redirects: Vec<Token<'a>>,
}
pub struct Pipeline<'a> {
      pub commands: Vec<SimpleCommand<'a>>,
}
pub struct List<'a> {
      pub items: Vec<(Pipeline<'a>, Separator)>,
}

pub fn ParseCommands() {
      //
}

pub fn ParsePipeline() {
      //
}

pub fn ParseList<'a>(tklist: &[Token<'a>]) -> Result<List<'a>, String> {
      let _ = tklist;

      /*
      let list: List;
      while tklist != TK_EOF {
          //
      }
      */

      Ok(List { items: Vec::new() })
}
