// src/main.rs

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use blindtest::load_project;

#[derive(Parser, Debug)]
#[command(name = "blindtest")]
#[command(about = "Blindtest video builder")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Render a blindtest from a JSON description
    Render {
        /// Path to the JSON montage file
        input: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Render { input } => {
            let project = load_project(&input)?;
            dbg!(project);
        }
    }

    Ok(())
}

