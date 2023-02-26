extern crate ansi_term;
extern crate clap;
extern crate dirs;
extern crate git2;
extern crate tico;

use clap::{App, AppSettings};

mod defaults;
mod precmd;
mod prompt;
mod rprompt;

fn main() {
    human_panic::setup_panic!();

    let matches = App::new("Turs")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(precmd::cli_arguments())
        .subcommand(prompt::cli_arguments())
        .subcommand(rprompt::cli_arguments())
        .get_matches();

    match matches.subcommand() {
        Some(("precmd", sub_matches)) => precmd::display(sub_matches),
        Some(("prompt", sub_matches)) => prompt::display(sub_matches),
        Some(("rprompt", sub_matches)) => rprompt::display(sub_matches),
        _ => (),
    }
}
