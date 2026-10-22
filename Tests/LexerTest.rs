/*
** MSH - Munch Shell
** Copyright (c) 2026-present TwilightUnix and contributors
** SPDX-License-Identifier: Apache-2.0
*/

/* LexerTest - lexer cases that the parser and executor depend on. */

#![allow(non_snake_case)]

use msh::TTY::Lexer::{TokenKind, Tokenize};

#[test]
fn WordsAndOperators() {
      let tokens = Tokenize("ls -l | grep rs > out.txt");

      assert_eq!(tokens.len(), 8); // 7 + EOF
      assert_eq!(tokens[0].tk, TokenKind::TK_Word);
      assert_eq!(tokens[0].val, "ls");
      assert_eq!(tokens[2].tk, TokenKind::TK_Pipe);
      assert_eq!(tokens[5].tk, TokenKind::TK_RABracket);
      assert_eq!(tokens[7].tk, TokenKind::TK_EOF);
}

#[test]
fn DoubleCharOperators() {
      let tokens = Tokenize("a >> b && c || d");

      assert_eq!(tokens[1].tk, TokenKind::TK_Append);
      assert_eq!(tokens[1].val, ">>");
      assert_eq!(tokens[3].tk, TokenKind::TK_And);
      assert_eq!(tokens[3].val, "&&");
      assert_eq!(tokens[5].tk, TokenKind::TK_Or);
      assert_eq!(tokens[5].val, "||");
}

#[test]
fn QuotedSpanStaysOneWord() {
      let tokens = Tokenize("echo \"a b\"");

      assert_eq!(tokens[1].tk, TokenKind::TK_Word);
      assert_eq!(tokens[1].val, "\"a b\"");
}

#[test]
fn AdjacentPartsJoinOneWord() {
      // plain + quoted + plain, no spaces between: one argument
      let tokens = Tokenize("echo a\"b c\"d");

      assert_eq!(tokens.len(), 3); // echo, a"b c"d, EOF
      assert_eq!(tokens[1].val, "a\"b c\"d");
}

#[test]
fn QuotesArePreserved() {
      let tokens = Tokenize("echo 'raw $VAR'");

      assert_eq!(tokens[1].val, "'raw $VAR'");
}

#[test]
fn VariableAndTildeStayInWords() {
      let tokens = Tokenize("echo $VAR $? ~/x");

      assert_eq!(tokens[1].val, "$VAR");
      assert_eq!(tokens[2].val, "$?");
      assert_eq!(tokens[3].val, "~/x");
      assert!(tokens.iter().all(|t| t.tk != TokenKind::TK_EOF || t.val.is_empty()));
}

#[test]
fn CommentOnlyAtWordStart() {
      let mid_word = Tokenize("echo foo#bar");
      assert_eq!(mid_word[1].val, "foo#bar");
      assert_eq!(mid_word[2].tk, TokenKind::TK_EOF);

      let at_start = Tokenize("echo foo #bar");
      assert_eq!(at_start[1].val, "foo");
      assert_eq!(at_start[2].tk, TokenKind::TK_EOF);
}

#[test]
fn EscapeJoinsAdjacentParts() {
      let tokens = Tokenize("echo foo\\ bar");

      assert_eq!(tokens.len(), 3);
      assert_eq!(tokens[1].val, "foo\\ bar");
}

#[test]
fn UnterminatedQuoteTakesRestOfLine() {
      let tokens = Tokenize("echo \"unclosed");

      assert_eq!(tokens[1].tk, TokenKind::TK_Word);
      assert_eq!(tokens[1].val, "\"unclosed");
}

#[test]
fn EmptyLineIsJustEof() {
      let tokens = Tokenize("");

      assert_eq!(tokens.len(), 1);
      assert_eq!(tokens[0].tk, TokenKind::TK_EOF);
}
