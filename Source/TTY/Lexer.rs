/*
** MSH - Munch Shell
** Copyright (c) 2026-present TwilightUnix and contributors
** SPDX-License-Identifier: Apache-2.0
*/

/* Lexer.rs - tty lexer for msh */

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
      TK_Word, // plain text: ls, -l, file.txt, VAR=x, *.rs, $VAR, ~/x

      /* Operators */
      TK_Semicolon,   // ";"  semicolon
      TK_LRParen,     // "("  left round parenthesis
      TK_RRParen,     // ")"  right round parenthesis
      TK_LCParen,     // "{"  left curly parenthesis
      TK_RCParen,     // "}"  right curly parenthesis
      TK_LABracket,   // "<"  left angle bracket (redirect in)
      TK_RABracket,   // ">"  right angle bracket (redirect out)
      TK_Append,      // ">>" append redirect
      TK_Exclamation, // "!"  exclamation mark (history)
      TK_Ampersand,   // "&"  ampersand (background)
      TK_And,         // "&&" and operator
      TK_Pipe,        // "|"  pipe
      TK_Or,          // "||" or operator

      TK_EOF, // End of file
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
      pub tk: TokenKind,
      pub val: &'a str,
}

/* Characters that start an operator token.      *
 * Quotes and '\' are word parts, not operators. */
fn IsOpStart(c: char) -> bool {
      matches!(c, ';' | '(' | ')' | '{' | '}' | '!' | '<' | '>' | '&' | '|')
}

pub fn Tokenize<'a>(buffer: &'a str) -> Vec<Token<'a>> {
      let mut tokens: Vec<Token<'a>> = Vec::new();
      let mut cur = buffer.char_indices().peekable();

      while let Some(&(i, c)) = cur.peek() {
            if c == ' ' || c == '\t' {
                  cur.next();
                  continue;
            }
            if c == '#' {
                  break; // comment: ignore rest of line
            }

            if IsOpStart(c) {
                  cur.next();
                  let next = cur.peek().map(|&(_, p)| p);
                  let (tk, len) = match (c, next) {
                        ('>', Some('>')) => {
                              cur.next();
                              (TokenKind::TK_Append, 2)
                        }
                        ('&', Some('&')) => {
                              cur.next();
                              (TokenKind::TK_And, 2)
                        }
                        ('|', Some('|')) => {
                              cur.next();
                              (TokenKind::TK_Or, 2)
                        }
                        ('>', _) => (TokenKind::TK_RABracket, 1),
                        ('<', _) => (TokenKind::TK_LABracket, 1),
                        ('&', _) => (TokenKind::TK_Ampersand, 1),
                        ('|', _) => (TokenKind::TK_Pipe, 1),
                        (';', _) => (TokenKind::TK_Semicolon, 1),
                        ('(', _) => (TokenKind::TK_LRParen, 1),
                        (')', _) => (TokenKind::TK_RRParen, 1),
                        ('{', _) => (TokenKind::TK_LCParen, 1),
                        ('}', _) => (TokenKind::TK_RCParen, 1),
                        ('!', _) => (TokenKind::TK_Exclamation, 1),
                        _ => unreachable!(),
                  };
                  tokens.push(Token {
                        tk,
                        val: &buffer[i..i + len],
                  });
                  continue;
            }

            // Word: adjacent parts (plain, quoted, escaped) join into one Word.
            // Quotes and '\' are kept in the text; expansion strips them later.
            let start = i;
            let mut end = i;

            while let Some(&(j, p)) = cur.peek() {
                  if p == ' ' || p == '\t' || IsOpStart(p) {
                        break;
                  }

                  if p == '\\' {
                        cur.next();
                        if let Some((k, q)) = cur.next() {
                              end = k + q.len_utf8();
                        } else {
                              end = j + p.len_utf8();
                        }
                        continue;
                  }

                  if p == '\'' || p == '"' {
                        cur.next(); // opening quote
                        end = j + p.len_utf8();
                        for (k, q) in cur.by_ref() {
                              end = k + q.len_utf8();
                              if q == p {
                                    break;
                              }
                        }
                        continue;
                  }

                  end = j + p.len_utf8();
                  cur.next();
            }

            tokens.push(Token {
                  tk: TokenKind::TK_Word,
                  val: &buffer[start..end],
            });
      }

      tokens.push(Token {
            tk: TokenKind::TK_EOF,
            val: "",
      });
      return tokens;
}
