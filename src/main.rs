extern crate clap;
mod checkers;

use checkers::*;
use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let matches = App::new("grmr")
        .version("0.1.0")
        .author("Janusz Marcinkiewicz <virrages@gmail.com>")
        .about("Grammar checking tool")
        .arg(Arg::with_name("weasel")
                               .long("weasel")
        .help("Check documents only for weasel words. Weasel words make sentence 'weaker'."))
        .arg(Arg::with_name("passive")
                               .long("passive").help("Check documents only for passive voice. Using passive voice can hide important informations."))
        .arg(Arg::with_name("ill")
                                       .long("ill")
.help("Check documents only for illusion/duplicate words like: \"... the the ...\" etc."))
.arg(Arg::with_name("FILE").multiple(true).required(true))
        .get_matches();

    let file_names = matches.values_of("FILE").unwrap();
    for file_name in file_names {
        let mut file = File::open(file_name)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

        Dups::check(file_name, lines.clone());
        println!("");
        Passive::check(file_name, lines.clone());
        println!("");
        Weasel::check(file_name, lines.clone());
        println!("");
    }

    Ok(())
}
