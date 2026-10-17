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

pub fn ParseCommands<'a>(
      tklist: &[Token<'a>],
      pos: &mut usize,
) -> Result<SimpleCommand<'a>, Error> {
      let _ = (tklist, pos);
      Ok(SimpleCommand {
            args: Vec::new(),
            redirects: Vec::new(),
      })
}

/* If TK_Pipe '|' found in a commandline, combine "cmd1 | cmd2" combine both commands */
pub fn ParsePipeline<'a>(tklist: &[Token<'a>], pos: &mut usize) -> Result<Pipeline<'a>, Error> {
      let mut commands = Vec::new();

      loop {
            commands.push(ParseCommands(tklist, pos)?);

            if tklist[*pos].tk == TokenKind::TK_Pipe {
                  *pos += 1; // consume '|'
                  if tklist[*pos].tk == TokenKind::TK_EOF {
                        return Err(Error::ERR_UnexpectedEOF); // example senerio: a |
                  }
                  if tklist[*pos].tk == TokenKind::TK_Pipe {
                        // senerio: a | | b
                        return Err(Error::ERR_Syntax(tklist[*pos].val.to_string()));
                  }
            } else {
                  break; // ';', '&', '&&', '||', EOF
            }
      }

      Ok(Pipeline { commands })
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
