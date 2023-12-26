use clap::Parser;
use std::io;

mod cli;

fn main() -> io::Result<()> {
    cli::Cli::parse();

    Ok(())
}
