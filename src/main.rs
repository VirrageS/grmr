extern crate clap;
extern crate glob;
mod checkers;

use checkers::*;
use clap::{App, Arg};
use glob::glob;
use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::result::Result<(), Box<error::Error>> {
    let matches = App::new("grmr")
        .version("0.1.0")
        .author("Janusz Marcinkiewicz <virrages@gmail.com>")
        .about("Grammar checking tool")
        .arg(
            Arg::with_name("weasel")
                .long("weasel")
                .help("Check documents only for weasel words. Weasel words make sentence 'weaker'."),
        )
        .arg(
            Arg::with_name("passive")
                .long("passive")
                .help("Check documents only for passive voice. Using passive voice can hide important informations."),
        )
        .arg(
            Arg::with_name("ill")
                .long("ill")
                .help("Check documents only for illusion/duplicate words like: \"... the the ...\" etc."),
        )
        .arg(Arg::with_name("FILE").multiple(true).required(true))
        .get_matches();

    // By default use all checkers
    let mut to_check: Vec<CheckerType> = vec![CheckerType::Weasel, CheckerType::Passive, CheckerType::Dups];
    if matches.is_present("weasel") || matches.is_present("passive") || matches.is_present("ill") {
        to_check = vec![];
        if matches.is_present("weasel") {
            to_check.push(CheckerType::Weasel);
        }
        if matches.is_present("passive") {
            to_check.push(CheckerType::Passive);
        }
        if matches.is_present("ill") {
            to_check.push(CheckerType::Dups);
        }
    }

    let patterns = matches.values_of("FILE").unwrap();
    let checkers = Checkers::new();
    for pattern in patterns {
        for path in glob(pattern)? {
            if let Some(path_cleaned) = path?.to_str() {
                let mut file = File::open(path_cleaned)?;
                let reader = BufReader::new(file);
                let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
                checkers.check(&to_check, path_cleaned, lines);
            }
        }
    }

    Ok(())
}
