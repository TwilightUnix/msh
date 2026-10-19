/*
** MSH - Munch Shell
** Copyright (c) 2026-present TwilightUnix and contributors
** SPDX-License-Identifier: Apache-2.0
*/

/* Lexer.rs - tty lexer for  */

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
      TK_Word, // plain text: ls, -l, file.txt, VAR=x, *.rs

      /* Operators */
      TK_BSlash,      // "\"  back slash (escape)
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

pub fn Tokenize<'a>(buffer: &'a str) -> Vec<Token<'a>> {
      let mut tokens: Vec<Token<'a>> = Vec::new();
      let mut cur = buffer.char_indices().peekable();

      while let Some((i, c)) = cur.next() {
            match c {
                  ' ' | '\t' => continue,
                  '#' => break, // comment: ignore rest of line
                  ';' => tokens.push(Token {
                        tk: TokenKind::TK_Semicolon,
                        val: &buffer[i..i + 1],
                  }),
                  '(' => tokens.push(Token {
                        tk: TokenKind::TK_LRParen,
                        val: &buffer[i..i + 1],
                  }),
                  ')' => tokens.push(Token {
                        tk: TokenKind::TK_RRParen,
                        val: &buffer[i..i + 1],
                  }),
                  '{' => tokens.push(Token {
                        tk: TokenKind::TK_LCParen,
                        val: &buffer[i..i + 1],
                  }),
                  '}' => tokens.push(Token {
                        tk: TokenKind::TK_RCParen,
                        val: &buffer[i..i + 1],
                  }),
                  '!' => tokens.push(Token {
                        tk: TokenKind::TK_Exclamation,
                        val: &buffer[i..i + 1],
                  }),
                  '$' | '~' => {
                        // start of a word: expansion happens later, keep '$'/'~' in the text
                        let mut end = i + c.len_utf8();
                        while let Some(&(j, p)) = cur.peek() {
                              if p == ' ' || p == '\t' || "|<>&;(){}!'\"\\".contains(p) {
                                    break;
                              }
                              end = j + p.len_utf8();
                              cur.next();
                        }
                        tokens.push(Token {
                              tk: TokenKind::TK_Word,
                              val: &buffer[i..end],
                        });
                  }
                  '<' => tokens.push(Token {
                        tk: TokenKind::TK_LABracket,
                        val: &buffer[i..i + 1],
                  }),
                  '>' => {
                        if matches!(cur.peek(), Some((_, '>'))) {
                              cur.next();
                              tokens.push(Token {
                                    tk: TokenKind::TK_Append,
                                    val: &buffer[i..i + 2],
                              });
                        } else {
                              tokens.push(Token {
                                    tk: TokenKind::TK_RABracket,
                                    val: &buffer[i..i + 1],
                              });
                        }
                  }
                  '&' => {
                        if matches!(cur.peek(), Some((_, '&'))) {
                              cur.next();
                              tokens.push(Token {
                                    tk: TokenKind::TK_And,
                                    val: &buffer[i..i + 2],
                              });
                        } else {
                              tokens.push(Token {
                                    tk: TokenKind::TK_Ampersand,
                                    val: &buffer[i..i + 1],
                              });
                        }
                  }
                  '|' => {
                        if matches!(cur.peek(), Some((_, '|'))) {
                              cur.next();
                              tokens.push(Token {
                                    tk: TokenKind::TK_Or,
                                    val: &buffer[i..i + 2],
                              });
                        } else {
                              tokens.push(Token {
                                    tk: TokenKind::TK_Pipe,
                                    val: &buffer[i..i + 1],
                              });
                        }
                  }
                  '\\' => {
                        // escape: next char joins a Word literally (e.g. "\ " stays one word)
                        if let Some((j, p)) = cur.next() {
                              tokens.push(Token {
                                    tk: TokenKind::TK_Word,
                                    val: &buffer[i..j + p.len_utf8()],
                              });
                        } else {
                              tokens.push(Token {
                                    tk: TokenKind::TK_BSlash,
                                    val: &buffer[i..i + 1],
                              });
                        }
                  }
                  '\'' | '"' => {
                        // quoted span is one Word; keep quotes, expansion strips them later
                        let quote = c;
                        let mut end = i + c.len_utf8();
                        for (j, p) in cur.by_ref() {
                              end = j + p.len_utf8();
                              if p == quote {
                                    break;
                              }
                        }
                        tokens.push(Token {
                              tk: TokenKind::TK_Word,
                              val: &buffer[i..end],
                        });
                  }
                  _ => {
                        // word: consume until space or operator char
                        let mut end = i + c.len_utf8();
                        while let Some(&(j, p)) = cur.peek() {
                              if p == ' ' || p == '\t' || "|<>&;(){}!'\"\\".contains(p) {
                                    break;
                              }
                              end = j + p.len_utf8();
                              cur.next();
                        }
                        tokens.push(Token {
                              tk: TokenKind::TK_Word,
                              val: &buffer[i..end],
                        });
                  }
            }
      }

      tokens.push(Token {
            tk: TokenKind::TK_EOF,
            val: "",
      });
      return tokens;
}
