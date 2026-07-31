// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use clap_lex::RawArgs;
use cosmic_greeter::{greeter, locker};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_args = RawArgs::from_args();
    let mut cursor = raw_args.cursor();

    // Parse the arguments
    while let Some(arg) = raw_args.next_os(&mut cursor) {
        match arg.to_str() {
            Some("--help") | Some("-h") => {
                print_help(env!("CARGO_PKG_VERSION"), env!("VERGEN_GIT_SHA"));
                return Ok(());
            }
            Some("--version") | Some("-v") => {
                println!(
                    "wmde-greeter {} (git commit {})",
                    env!("CARGO_PKG_VERSION"),
                    env!("VERGEN_GIT_SHA")
                );
                return Ok(());
            }
            _ => {}
        }
    }

    match pwd::Passwd::current_user() {
        Some(current_user) => match current_user.name.as_str() {
            "wmde-greeter" => greeter::main(),
            _ => locker::main(current_user),
        },
        _ => Err("failed to determine current user".into()),
    }
}

fn print_help(version: &str, git_rev: &str) {
    println!(
        r#"wmde-greeter {version} (git commit {git_rev})
WMDE <https://wmde.fun>

Designed for the WMDE desktop environment, wmde-greeter is a libcosmic
frontend for greetd which can be run inside of wmde-comp.

Project home page: https://github.com/Lin-WMDE/wmde-greeter

Options:
  -h, --help     Show this message
  -v, --version  Show the version of wmde-greeter"#
    );
}
