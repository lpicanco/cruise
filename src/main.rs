use crate::config::{Cli, Commands};
use clap::Parser;

mod config;
mod simple;
mod simple3;
mod start;
mod stop;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start => start::start(),
        Commands::Stop => stop::stop(),
    }
}
