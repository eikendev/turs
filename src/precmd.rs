use std::env;
use ansi_term::Colour::Fixed;
use clap::{ArgMatches, App, SubCommand};
use tico::tico;

use super::defaults;

fn shorten_path(cwd: &str) -> String {
    let path = match dirs::home_dir() {
        Some(path) => match path.to_str() {
            Some(path) => cwd.replace(path, "~"),
            _ => return String::from(""),
        },
        _ => return String::from(""),
    };

    tico(&path)
}

pub fn display(_matches: &ArgMatches) {
    let path = match env::current_dir() {
        Ok(path) => shorten_path(path.to_str().unwrap()),
        _ => String::from("???"),
    };

    let path = Fixed(defaults::color::BLUE).paint(path);

    println!("");
    println!("{}", path);
}

pub fn cli_arguments<'a>() -> App<'a, 'a> {
    SubCommand::with_name("precmd")
}
