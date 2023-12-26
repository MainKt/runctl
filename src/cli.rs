use clap::Parser;

mod subcommands;

use subcommands::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}
