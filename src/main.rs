extern crate clap;
extern crate glob;
mod checkers;

use checkers::*;
use clap::{App, Arg};
use glob::glob;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let matches = App::new("grmr")
        .version("0.1.0")
        .author("Janusz Marcinkiewicz <virrages@gmail.com>")
        .about("Grammar checking tool")
        .arg(Arg::with_name("weasel").long("weasel").help("Check documents only for weasel words. Weasel words make sentence 'weaker'."))
        .arg(Arg::with_name("passive").long("passive").help("Check documents only for passive voice. Using passive voice can hide important informations."))
        .arg(Arg::with_name("ill").long("ill").help("Check documents only for illusion/duplicate words like: \"... the the ...\" etc."))
        .arg(Arg::with_name("FILE").multiple(true).required(true))
        .get_matches();

    let patterns = matches.values_of("FILE").unwrap();
    let checkers = Checkers::new();
    for pattern in patterns {
        // TODO: fix this `unwrap` circus...
        for path in glob(pattern).unwrap() {
            let unwrapped_path = path.unwrap();
            let path_cleaned = unwrapped_path.to_str().unwrap();
            let mut file = File::open(path_cleaned)?;
            let reader = BufReader::new(file);
            let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
            checkers.check_all(path_cleaned, lines);
        }
    }

    Ok(())
}
