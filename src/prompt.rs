use clap::{App, Arg, ArgMatches, SubCommand};

use super::defaults;

const COMMAND_KEYMAP: &str = "vicmd";
const NO_ERROR: &str = "0";

pub fn display(matches: &ArgMatches) {
    let last_return_code = matches.value_of("last_return_code").unwrap_or("0");
    let keymap = matches.value_of("keymap").unwrap_or("US");

    let symbol = match keymap {
        COMMAND_KEYMAP => defaults::symbol::COMMAND,
        _ => defaults::symbol::INSERT,
    };

    let color = match (symbol, last_return_code) {
        (defaults::symbol::COMMAND, _) => defaults::color::YELLOW,
        (_, NO_ERROR) => defaults::color::GREEN,
        _ => defaults::color::RED,
    };

    print!("%F{{{color}}}{symbol}%f ");
}

pub fn cli_arguments<'a>() -> App<'a> {
    SubCommand::with_name("prompt")
        .arg(Arg::with_name("last_return_code").short('r').takes_value(true))
        .arg(Arg::with_name("keymap").short('k').takes_value(true))
}
