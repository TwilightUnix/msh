/*
** MSH - Munch Shell
** Copyright (c) 2026-present Tanvir
** SPDX-License-Identifier: Apache-2.0
*/

fn PrintHelp() {
      println!("Munch shell (msh) - Simple shell for linux.");
      println!("Usage: msh [OPTION]");
      println!();
      println!("Options:");
      println!("  -h, --help         Print this help and exit");
      println!("  -v, --version      Print version and exit");
      println!();
      println!("With no OPTION, start an interactive shell.");
      println!();
      println!("Report issues at: https://github.com/tanvir-techbro/msh/issues");
}

pub fn ParseOption(option: Vec<String>) {
      if option.len() == 2 {
            if option[1] == "-h" || option[1] == "--help" {
                  PrintHelp();
            }
      }
}
