/*
** MSH - Munch Shell
** Copyright (c) 2026-present Tanvir
** SPDX-License-Identifier: Apache-2.0
*/

#[derive(PartialEq)]
enum TokenKind {
      TK_Word, // plain text: ls, -l, file.txt, VAR=x, *.rs

      /* Operators */
      TK_BSlash,      // "\" back slash (escape)
      TK_Semicolon,   // ";" semicolon
      TK_LRParen,     // "(" left round parenthesis
      TK_RRParen,     // ")" right round parenthesis
      TK_LCParen,     // "{" left curly parenthesis
      TK_RCParen,     // "}" right curly parenthesis
      TK_LABracket,   // "<" left angle bracket (redirect in)
      TK_RABracket,   // ">" right angle bracket (redirect out)
      TK_Append,      // ">>" append redirect
      TK_Exclamation, // "!" exclamation mark (history)
      TK_Dollar,      // "$" dollar sign (variable)
      TK_Ampersand,   // "&" ampersand (background)
      TK_And,         // "&&" and operator
      TK_Pipe,        // "|" pipe
      TK_Or,          // "||" or operator
      TK_Tilde,       // "~" tilde (home)
      TK_SQuote,      // ' single quotation mark
      TK_DQuote,      // " double quotation mark
      TK_Hash,        // "#" hash (comment)

      TK_EOF, // End of line
}

pub struct Token<'a> {
      pub tk: TokenKind,
      pub val: &'a str,
}

pub fn Tokenize<'a>(buffer: &'a str) -> Vec<Token<'a>> {
      let mut tokens: Vec<Token<'a>> = Vec::new();
      let mut cur = buffer.chars().peekable();

      while let Some(c) = cur.next() {
            if c == ' ' {
                  continue;
            }
      }

      return tokens;
}
