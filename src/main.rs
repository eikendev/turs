extern crate ansi_term;
extern crate clap;
extern crate dirs;
extern crate git2;
extern crate tico;

use clap::{App, AppSettings};

mod precmd;
mod prompt;
mod rprompt;
mod defaults;

fn main() {
    let matches = App::new("Turs")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(precmd::cli_arguments())
        .subcommand(prompt::cli_arguments())
        .subcommand(rprompt::cli_arguments())
        .get_matches();

    match matches.subcommand() {
        ("precmd", Some(sub_matches)) => precmd::display(sub_matches),
        ("prompt", Some(sub_matches)) => prompt::display(sub_matches),
        ("rprompt", Some(sub_matches)) => rprompt::display(sub_matches),
        _ => (),
    }
}
