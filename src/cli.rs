use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Precmd {},
    Prompt {
        #[arg(short = 'k', default_value = "0")]
        keymap: String,

        #[arg(short = 'r', default_value = "US")]
        last_return_code: String,
    },
    Rprompt {},
}
