// Copyright 2023 Thomas Obernosterer (https://atjon.tv).
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod day01;
mod day02;
mod util;

use crate::day01::Day01;
use crate::day02::Day02;
use crate::util::boxed;
use clap::{value_parser, Arg, ArgAction, Command};
use std::io;
use std::path::PathBuf;

pub trait Challenge {
    fn new() -> Self
    where
        Self: Sized;
    fn id(&self) -> &String;
    fn run(&self, challenge: &str, file: &PathBuf) -> io::Result<bool>;
}

fn main() -> io::Result<()> {
    let available_challenges: Vec<Box<dyn Challenge>> = vec![boxed(Day01::new())];

    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("challenge")
                .short('c')
                .help("The Challenge to solve in the format d01p1 for Day 1 Part 1")
                .required(true),
        )
        .arg(
            Arg::new("inputs")
                .help("Path to the input files")
                .short('i')
                .action(ArgAction::Append)
                .value_parser(value_parser!(PathBuf))
                .required(true),
        )
        .get_matches();

    let challenge = matches.get_one::<String>("challenge").unwrap();
    let inputs = matches.get_many::<PathBuf>("inputs").unwrap();

    for c in &available_challenges {
        if challenge.starts_with(c.id()) {
            let part = challenge.split(c.id()).nth(1).unwrap();
            for input in inputs.clone().into_iter() {
                c.run(part, input).map_err(|e| {
                    eprintln!(
                        "Failed to run challenge {} on input {}: {}",
                        c.id(),
                        input.display(),
                        e
                    );
                    e
                })?;
            }
        }
    }
    Ok(())
}
