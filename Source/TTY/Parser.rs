/*
** MSH - Munch Shell
** Copyright (c) 2026-present Tanvir
** SPDX-License-Identifier: Apache-2.0
*/

/* Parser.rs - tty parser for msh */

use crate::TTY::Error::Error;
use crate::TTY::Lexer::{Token, TokenKind};

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

pub fn ParsePipeline<'a>(tklist: &[Token<'a>], pos: &mut usize) -> Result<Pipeline<'a>, Error> {
      Ok(Pipeline {
            commands: Vec::new(),
      })
}

/* Entry function. Parse list of Token pass the list to ParsePipeline */
pub fn ParseList<'a>(tklist: &[Token<'a>]) -> Result<List<'a>, Error> {
      let mut pos: usize = 0;
      let mut list = List { items: Vec::new() };

      while pos < tklist.len() && tklist[pos].tk != TokenKind::TK_EOF {
            let pipe = ParsePipeline(tklist, &mut pos)?;
            if pos >= tklist.len() {
                  return Err(Error::ERR_UnexpectedEOF);
            }

            match tklist[pos].tk {
                  TokenKind::TK_Semicolon => {
                        pos += 1;
                        list.items.push((pipe, Separator::SP_Semicolon));
                  }
                  TokenKind::TK_Ampersand => {
                        pos += 1;
                        list.items.push((pipe, Separator::SP_Background));
                  }
                  TokenKind::TK_And => {
                        pos += 1;
                        list.items.push((pipe, Separator::SP_And));
                  }
                  TokenKind::TK_Or => {
                        pos += 1;
                        list.items.push((pipe, Separator::SP_Or));
                  }
                  TokenKind::TK_EOF => {
                        // last pipeline; separator unused, Semicolon as dummy
                        list.items.push((pipe, Separator::SP_Semicolon));
                        break;
                  }
                  _ => return Err(Error::ERR_Syntax(tklist[pos].val.to_string())),
            }
      }

      return Ok(list);
}
