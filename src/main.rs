use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use blindtest::{ffmpeg, load_project};
use blindtest::ffmpeg_command::build_ffmpeg_command;
use blindtest::validate::validate_project;

#[derive(Parser, Debug)]
#[command(
    name = "blindtest",
    version,
    about = "Génère automatiquement une vidéo de blind test.",
    long_about = "BlindTestAuto\n\n\
Génère une vidéo de blind test à partir de clips vidéo.\n\
Chaque clip comporte deux phases :\n\
- Devinette : écran noir + musique + minuteur\n\
- Révélation : vidéo + réponse affichée\n\n\
Deux modes d'utilisation :\n\
- Mode guidé : assistant interactif pour créer le projet\n\
- Mode rapide : un dossier de vidéos suffit\n",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(
        about = "Rendre une vidéo à partir d'un fichier JSON",
        long_about = "Lit un fichier JSON (format V1), valide le projet et lance FFmpeg.\n\n\
Exemples :\n\
  blindtest render montage.json\n\
  blindtest render montage.json --dry-run\n"
    )]
    Render {
        #[arg(
            value_name = "JSON",
            help = "Chemin vers le fichier JSON (ex: montage.json)"
        )]
        input: PathBuf,

        #[arg(
            long,
            help = "Affiche la commande FFmpeg sans lancer le rendu"
        )]
        dry_run: bool,
    },

    #[command(
        about = "Créer un blind test (mode guidé ou mode rapide)",
        long_about = "La commande `new` permet de créer un blind test de deux façons :\n\n\
MODE GUIDÉ (par défaut)\n\
  blindtest new\n\
  → Pose des questions pour générer un fichier JSON.\n\n\
MODE RAPIDE (--quick)\n\
  blindtest new --quick DOSSIER\n\
  → Utilise tous les fichiers .mp4 du dossier.\n\
  → Le nom des fichiers devient la réponse.\n\
  → Les durées et le rendu utilisent des valeurs par défaut.\n\
  → La vidéo est générée automatiquement.\n\n\
Options du mode rapide :\n\
  --shuffle     Mélange l'ordre des clips\n\
  --only-json   Génère uniquement le JSON (pas de rendu)\n\
  --dry-run     Affiche la commande FFmpeg sans lancer le rendu\n\n\
Exemples :\n\
  blindtest new\n\
  blindtest new --quick ./videos\n\
  blindtest new --quick ./videos --shuffle\n\
  blindtest new --quick ./videos --only-json\n"
    )]
    New {
        #[arg(
            long,
            help = "Mode rapide : utilise tous les fichiers .mp4 d'un dossier"
        )]
        quick: bool,

        #[arg(
            long,
            help = "Mélange l'ordre des clips (utile avec --quick)"
        )]
        shuffle: bool,

        #[arg(
            long,
            help = "Avec --quick : génère uniquement le JSON, sans rendre la vidéo"
        )]
        only_json: bool,

        #[arg(
            long,
            help = "Avec --quick : affiche la commande FFmpeg sans rendre la vidéo"
        )]
        dry_run: bool,

        #[arg(
            value_name = "DOSSIER",
            help = "Dossier contenant les fichiers .mp4",
            requires = "quick"
        )]
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
            println!("🎬 Vidéo générée : {}", project.output.path);
        }

        Commands::New {
            quick,
            shuffle,
            only_json,
            dry_run,
            folder,
        } => {
            let (project, json_path) = if quick {
                let folder = folder
                    .context("Avec --quick, vous devez fournir un dossier (ex: ./videos)")?;
                blindtest::wizard::run_quick(folder, shuffle)?
            } else {
                blindtest::wizard::run_new_wizard()?
            };

            validate_project(&project)?;

            blindtest::wizard::write_project_json(&json_path, &project)?;
            println!("✅ JSON généré : {}", json_path);

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

