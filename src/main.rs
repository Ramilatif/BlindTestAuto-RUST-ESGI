// src/main.rs

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use blindtest::ffmpeg;
use blindtest::ffmpeg_command::build_ffmpeg_command;
use blindtest::load_project;
use blindtest::validate::validate_project;

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

        /// Print the ffmpeg command without running it
        #[arg(long)]
        dry_run: bool,
    },
    /// Interactive wizard to generate a montage JSON (V1 format)
    New {
        #[arg(long)]
        quick: bool,
        /// Dossier contenant les clips vidéo (requis avec --quick)
        folder: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Render { input, dry_run } => {
            let project = load_project(&input)?;
            validate_project(&project)?;

            let spec = build_ffmpeg_command(&project)?;

            if dry_run {
                println!("{}", ffmpeg::format_command(&spec));
                return Ok(());
            }

            ffmpeg::run(&spec)?;
        }
        Commands::New { quick, folder } => {
            let (project, json_path) = if quick {
                let folder = folder.context("Avec --quick, vous devez fournir un dossier")?;
                blindtest::wizard::run_quick(folder)?
            } else {
                blindtest::wizard::run_new_wizard()?
            };

            blindtest::wizard::write_project_json(&json_path, &project)?;
            println!("✅ JSON généré : {}", json_path);
        }
    }
    Ok(())
}
