/*
** MSH - Munch Shell
** Copyright (c) 2026-present Tanvir
** SPDX-License-Identifier: Apache-2.0
*/

pub fn ParseOption(option: Vec<String>) {
      if option.len() == 2 {
            if option[1] == "-h" || option[1] == "--help" {
                  println!("Munch shell. Usage: ");
            }
      }
}
