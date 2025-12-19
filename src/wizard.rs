use crate::model::{Clip, Intro, Output, Project, Timings};
use crate::timecode::parse_timecode_ms;
use anyhow::{Context, Result, bail};
use inquire::{Confirm, Text};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;
use std::path::{Path, PathBuf};

/// Wizard interactif (assistant guidé)
pub fn run_new_wizard() -> Result<(Project, String)> {
    let json_path = Text::new("Chemin du fichier JSON à générer ?")
        .with_default("montage.json")
        .prompt()?;

    // --- INTRO (optionnelle) ---
    let add_intro =
        Confirm::new("Ajouter une introduction (image + titre + musique) avant le blindtest ?")
            .with_default(true)
            .prompt()?;

    let intro: Option<Intro> = if add_intro {
        let background = Text::new("Chemin de l'image de fond (ex: assets/intro.png) ?")
            .with_default("assets/intro.png")
            .prompt()?;

        let title = Text::new("Titre affiché pendant l'intro ?")
            .with_default("Blind Test")
            .prompt()?;

        let music = Text::new("Chemin de la musique d'intro (ex: assets/intro.mp3) ?")
            .with_default("assets/intro.mp3")
            .prompt()?;

        let duration = prompt_timecode("Durée de l'intro (HH:MM:SS.mmm) ?", "00:00:05.000")?;

        Some(Intro {
            background: background.trim().to_string(),
            title: title.trim().to_string(),
            music: music.trim().to_string(),
            duration,
        })
    } else {
        None
    };

    // --- OUTPUT ---
    let output_path = Text::new("Fichier vidéo de sortie ?")
        .with_default("render/blindtest.mp4")
        .prompt()?;

    let resolution = Text::new("Résolution (optionnel, ex: 1280x720) (laisser vide pour défaut)")
        .with_default("")
        .prompt()?;
    let resolution = (!resolution.trim().is_empty()).then(|| resolution.trim().to_string());

    let fps = Text::new("FPS (optionnel, ex: 30) (laisser vide pour défaut)")
        .with_default("")
        .prompt()?;
    let fps = if fps.trim().is_empty() {
        None
    } else {
        Some(fps.trim().parse::<u32>().context("FPS invalide")?)
    };

    // --- TIMINGS ---
    let guess_duration = prompt_timecode("Durée devinette (HH:MM:SS.mmm)", "00:00:10.000")?;
    let reveal_duration = prompt_timecode("Durée révélation (HH:MM:SS.mmm)", "00:00:05.000")?;

    // --- CLIPS ---
    let mut clips: Vec<Clip> = Vec::new();
    loop {
        let add = Confirm::new("Ajouter un clip ?")
            .with_default(clips.is_empty())
            .prompt()?;
        if !add {
            break;
        }

        let video = Text::new("Chemin de la vidéo")
            .with_default("videos/clip.mp4")
            .prompt()?;

        let start = prompt_timecode("Timecode de départ (HH:MM:SS.mmm)", "00:00:00.000")?;

        // UX: propose par défaut le nom de fichier comme réponse (super pratique)
        let default_answer = Path::new(video.trim())
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Artiste - Titre")
            .to_string();

        let answer = Text::new("Réponse à afficher (titre / artiste)")
            .with_default(&default_answer)
            .prompt()?;

        clips.push(Clip {
            video: video.trim().to_string(),
            start,
            answer: answer.trim().to_string(),
        });
    }

    let project = Project {
        intro,
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

/// Mode quick : dossier -> Project par défaut
pub fn run_quick(folder: PathBuf, shuffle: bool) -> Result<(Project, String)> {
    if !folder.exists() || !folder.is_dir() {
        bail!("Dossier invalide : {}", folder.display());
    }

    let mut files: Vec<PathBuf> = fs::read_dir(&folder)
        .with_context(|| format!("Impossible de lire {}", folder.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .and_then(|e| e.to_str())
                .map(|e| e.eq_ignore_ascii_case("mp4"))
                .unwrap_or(false)
        })
        .collect();

    if files.is_empty() {
        bail!("Aucun fichier .mp4 trouvé dans {}", folder.display());
    }

    files.sort();
    if shuffle {
        files.shuffle(&mut thread_rng());
    }

    let clips: Vec<Clip> = files
        .iter()
        .map(|p| Clip {
            video: p.to_string_lossy().to_string(),
            start: "00:00:00.000".into(),
            answer: p
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown")
                .to_string(),
        })
        .collect();

    let project = Project {
        intro: None,
        output: Output {
            path: "render/blindtest.mp4".into(),
            resolution: Some("1280x720".into()),
            fps: Some(30),
        },
        timings: Timings {
            guess_duration: "00:00:10.000".into(),
            reveal_duration: "00:00:05.000".into(),
        },
        clips,
    };

    Ok((project, "montage.json".into()))
}

/// Écriture du JSON pretty
pub fn write_project_json<P: AsRef<Path>>(path: P, project: &Project) -> Result<()> {
    let json = serde_json::to_string_pretty(project).context("impossible de sérialiser le JSON")?;
    let path = path.as_ref();

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).ok();
        }
    }

    fs::write(path, json).with_context(|| format!("Impossible d'écrire {}", path.display()))?;
    Ok(())
}

fn prompt_timecode(question: &str, default: &str) -> Result<String> {
    loop {
        let tc = Text::new(question).with_default(default).prompt()?;
        let tc = tc.trim().to_string();

        if parse_timecode_ms(&tc).is_ok() {
            return Ok(tc);
        }

        eprintln!("❌ Format invalide (ex: 00:00:10.000)");
    }
}
