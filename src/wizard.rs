// src/wizard.rs

use crate::model::{Clip, Output, Project, Timings};
use crate::timecode::parse_timecode_ms;
use anyhow::{Context, Result};
use inquire::{Confirm, Text};
use std::fs;
use std::path::Path;

pub fn run_new_wizard() -> Result<(Project, String)> {
    // Où écrire le JSON
    let json_path = Text::new("Chemin du fichier JSON à générer ?")
        .with_default("montage.json")
        .prompt()?;

    // Output vidéo
    let output_path = Text::new("Fichier vidéo de sortie (output.path) ?")
        .with_default("render/blindtest.mp4")
        .prompt()?;

    // Optionnels
    let resolution = Text::new("Résolution (optionnel, ex: 1280x720) ?")
        .with_default("")
        .prompt()?;
    let resolution = if resolution.trim().is_empty() {
        None
    } else {
        Some(resolution.trim().to_string())
    };

    let fps = Text::new("FPS (optionnel, ex: 30) ?")
        .with_default("")
        .prompt()?;
    let fps = if fps.trim().is_empty() {
        None
    } else {
        Some(
            fps.trim()
                .parse::<u32>()
                .context("FPS invalide (doit être un entier)")?,
        )
    };

    // Timings (format strict)
    let guess_duration = prompt_timecode("Durée devinette (HH:MM:SS.mmm) ?", "00:00:10.000")?;
    let reveal_duration = prompt_timecode("Durée révélation (HH:MM:SS.mmm) ?", "00:00:05.000")?;

    // Clips
    let mut clips: Vec<Clip> = Vec::new();
    loop {
        let add = Confirm::new("Ajouter un clip ?")
            .with_default(clips.is_empty())
            .prompt()?;
        if !add {
            break;
        }

        let video = Text::new("Chemin de la vidéo source ?")
            .with_default("videos/clip.mp4")
            .prompt()?;

        let start = prompt_timecode("Timecode de départ (HH:MM:SS.mmm) ?", "00:00:00.000")?;

        let answer = Text::new("Réponse à afficher (titre / artiste) ?")
            .with_default("Artiste - Titre")
            .prompt()?;

        clips.push(Clip {
            video: video.trim().to_string(),
            start,
            answer: answer.trim().to_string(),
        });
    }

    // Construire le Project (structure V1)
    let project = Project {
        output: Output {
            path: output_path.trim().to_string(),
            resolution,
            fps,
        },
        timings: Timings {
            guess_duration,
            reveal_duration,
        },
        clips,
    };

    Ok((project, json_path))
}

pub fn write_project_json<P: AsRef<Path>>(path: P, project: &Project) -> Result<()> {
    let s = serde_json::to_string_pretty(project).context("impossible de sérialiser le JSON")?;
    let path = path.as_ref();

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).ok(); // best-effort
        }
    }

    fs::write(path, s).with_context(|| format!("impossible d'écrire {}", path.display()))?;
    Ok(())
}

fn prompt_timecode(question: &str, default: &str) -> Result<String> {
    loop {
        let tc = Text::new(question).with_default(default).prompt()?;
        let tc = tc.trim().to_string();

        // On réutilise ton parser strict => UX propre
        if parse_timecode_ms(&tc).is_ok() {
            return Ok(tc);
        }

        eprintln!("❌ Format invalide. Exemple attendu: 00:00:10.000");
    }
}
