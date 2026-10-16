/*
** MSH - Munch Shell
** Copyright (c) 2026-present Tanvir
** SPDX-License-Identifier: Apache-2.0
*/

/* Error.rs - tty error types for msh */

pub enum Error {
      ERR_Unknown,
      ERR_Syntax(String), // unexpected token; holds its text
      ERR_UnexpectedEOF,  // line ended mid-construct
}

impl Error {
      pub fn Message(&self) -> String {
            match self {
                  Error::ERR_Unknown => "msh: unknown error".to_string(),
                  Error::ERR_Syntax(near) => format!("msh: syntax error near '{near}'"),
                  Error::ERR_UnexpectedEOF => "msh: syntax error near end of line".to_string(),
            }
      }
}

impl std::fmt::Display for Error {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.Message())
      }
}

impl std::fmt::Debug for Error {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.Message())
      }
}
