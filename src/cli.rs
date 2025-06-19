use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Weight Tracker")]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add today's weight entry
    Add {
        /// Body weight in kg (e.g. 70.2)
        weight: f32,
    },
}
