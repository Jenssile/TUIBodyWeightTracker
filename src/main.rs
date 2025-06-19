mod cli;
mod data;
mod tui;

use cli::{Cli, Commands};
use clap::Parser;
use data::write_entry;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add { weight }) => {
            write_entry(weight)?;
            println!("Weight added successfully.");
        }
        None => {
            tui::start_tui()?;
        }
    }

    Ok(())
}
