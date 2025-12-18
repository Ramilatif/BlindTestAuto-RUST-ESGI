use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use blindtest::{
    build_ffmpeg_command, ffmpeg, load_project, validate_project,
};

#[derive(Parser, Debug)]
#[command(name = "blindtest")]
#[command(about = "Automatic blind test video generator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Render a blindtest from a JSON file
    Render {
        input: PathBuf,

        #[arg(long)]
        dry_run: bool,
    },

    /// Create a new montage JSON (interactive or quick)
    New {
        /// Quick mode: generate from a folder of mp4 files
        #[arg(long)]
        quick: bool,

        /// Shuffle clips order (only meaningful with --quick)
        #[arg(long)]
        shuffle: bool,

        /// In quick mode: only generate JSON, do not render video
        #[arg(long)]
        only_json: bool,

        /// Print ffmpeg command without running it (quick mode)
        #[arg(long)]
        dry_run: bool,

        /// Folder containing video clips (required with --quick)
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

        Commands::New {
            quick,
            shuffle,
            only_json,
            dry_run,
            folder,
        } => {
            let (project, json_path) = if quick {
                let folder =
                    folder.context("Avec --quick, vous devez fournir un dossier")?;
                blindtest::wizard::run_quick(folder, shuffle)?
            } else {
                blindtest::wizard::run_new_wizard()?
            };

            // 1) Validation
            validate_project(&project)?;

            // 2) Écriture du JSON (toujours utile comme trace)
            blindtest::wizard::write_project_json(&json_path, &project)?;
            println!("✅ JSON généré : {}", json_path);

            // 3) Rendu automatique en quick
            if quick && !only_json {
                if let Some(parent) =
                    std::path::Path::new(&project.output.path).parent()
                {
                    if !parent.as_os_str().is_empty() {
                        std::fs::create_dir_all(parent).ok();
                    }
                }

                let spec = build_ffmpeg_command(&project)?;

                if dry_run {
                    println!("{}", ffmpeg::format_command(&spec));
                    return Ok(());
                }

                ffmpeg::run(&spec)?;
                println!("🎬 Vidéo générée : {}", project.output.path);
            }
        }
    }

    Ok(())
}

