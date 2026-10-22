/*
** MSH - Munch Shell
** Copyright (c) 2026-present TwilightUnix and contributors
** SPDX-License-Identifier: Apache-2.0
*/

/* ParserTest - grammar cases for v1: commands, pipelines, redirects, separators. */

#![allow(non_snake_case)]

use msh::TTY::Lexer::{TokenKind, Tokenize};
use msh::TTY::Parser::{ParseList, Separator};

fn Parse(line: &str) -> Result<msh::TTY::Parser::List<'_>, String> {
      // leak the line so the returned AST can outlive this helper
      let boxed: &'static str = Box::leak(line.to_string().into_boxed_str());
      let tokens = Tokenize(boxed);
      let tokens: &'static [msh::TTY::Lexer::Token<'static>] = Box::leak(tokens.into_boxed_slice());
      ParseList(tokens).map_err(|err| err.Message())
}

#[test]
fn SingleCommandParses() {
      let list = Parse("ls -l").unwrap();

      assert_eq!(list.items.len(), 1);
      let command = &list.items[0].0.commands[0];
      assert_eq!(command.args.len(), 2);
      assert_eq!(command.args[0].val, "ls");
      assert_eq!(command.args[1].val, "-l");
      assert!(command.redirects.is_empty());
}

#[test]
fn PipelineSplitsOnPipe() {
      let list = Parse("ls -l | grep rs").unwrap();

      assert_eq!(list.items.len(), 1);
      let pipeline = &list.items[0].0;
      assert_eq!(pipeline.commands.len(), 2);
      assert_eq!(pipeline.commands[0].args[0].val, "ls");
      assert_eq!(pipeline.commands[1].args[0].val, "grep");
      assert_eq!(pipeline.commands[1].args[1].val, "rs");
}

#[test]
fn RedirectIsCaptured() {
      let list = Parse("ls > out.txt").unwrap();
      let command = &list.items[0].0.commands[0];

      assert_eq!(command.redirects.len(), 1);
      assert_eq!(command.redirects[0].0.tk, TokenKind::TK_RABracket);
      assert_eq!(command.redirects[0].1.val, "out.txt");
}

#[test]
fn AppendRedirectIsCaptured() {
      let list = Parse("echo hi >> log").unwrap();
      let command = &list.items[0].0.commands[0];

      assert_eq!(command.redirects[0].0.tk, TokenKind::TK_Append);
      assert_eq!(command.redirects[0].1.val, "log");
}

#[test]
fn SeparatorsAreRecorded() {
      let list = Parse("ls; pwd && echo hi").unwrap();

      assert_eq!(list.items.len(), 3);
      assert!(matches!(list.items[0].1, Separator::SP_Semicolon));
      assert!(matches!(list.items[1].1, Separator::SP_And));
}

#[test]
fn BackgroundSeparator() {
      let list = Parse("sleep 10 & echo hi").unwrap();

      assert_eq!(list.items.len(), 2);
      assert!(matches!(list.items[0].1, Separator::SP_Background));
}

#[test]
fn VariableAndTildeParseAsArguments() {
      let list = Parse("echo $VAR ~/x").unwrap();
      let command = &list.items[0].0.commands[0];

      assert_eq!(command.args[1].val, "$VAR");
      assert_eq!(command.args[2].val, "~/x");
}

#[test]
fn DanglingPipeIsError() {
      assert!(Parse("ls |").is_err());
}

#[test]
fn EmptyPipelineStageIsError() {
      assert!(Parse("ls | | grep").is_err());
}

#[test]
fn LeadingPipeIsError() {
      assert!(Parse("| ls").is_err());
}

#[test]
fn DanglingAndIsError() {
      assert!(Parse("ls &&").is_err());
}

#[test]
fn DanglingOrIsError() {
      assert!(Parse("ls ||").is_err());
}

#[test]
fn RedirectWithoutTargetIsError() {
      assert!(Parse("ls >").is_err());
}

#[test]
fn RedirectOnlyCommandIsError() {
      assert!(Parse("> out.txt").is_err());
}

#[test]
fn EmptyLineGivesEmptyList() {
      let list = Parse("").unwrap();

      assert!(list.items.is_empty());
}
