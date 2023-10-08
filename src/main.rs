use clap::Parser;

mod cli;
mod defaults;
mod precmd;
mod prompt;
mod rprompt;

fn main() {
    human_panic::setup_panic!();

    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Precmd {} => precmd::display(),
        cli::Commands::Prompt {
            keymap,
            last_return_code,
        } => prompt::display(keymap, last_return_code),
        cli::Commands::Rprompt {} => rprompt::display(),
    }
}
