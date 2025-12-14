// src/lib.rs

pub mod model;
pub mod timecode;
pub mod validate;

use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::model::Project;

/// Load a Project from a JSON file on disk.
pub fn load_project<P: AsRef<Path>>(path: P) -> Result<Project> {
    let path_ref = path.as_ref();

    let mut file =
        File::open(path_ref).with_context(|| format!("failed to open JSON file: {}", path_ref.display()))?;

    // Read the whole file to provide better error messages if JSON is invalid.
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .with_context(|| format!("failed to read JSON file: {}", path_ref.display()))?;

    let project: Project = serde_json::from_str(&buf)
        .with_context(|| format!("invalid JSON in file: {}", path_ref.display()))?;

    Ok(project)
}

/// Load a Project from any reader (useful for unit tests).
pub fn load_project_from_reader<R: Read>(mut reader: R) -> Result<Project> {
    let mut buf = String::new();
    reader
        .read_to_string(&mut buf)
        .context("failed to read JSON from reader")?;

    let project: Project = serde_json::from_str(&buf).context("invalid JSON")?;
    Ok(project)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_project_from_reader_parses_valid_json() {
        let json = r#"
        {
          "output": { "path": "render/out.mp4", "resolution": "1920x1080", "fps": 30 },
          "timings": { "guess_duration": "00:00:10.000", "reveal_duration": "00:00:05.000" },
          "clips": [
            { "video": "videos/a.mp4", "start": "00:00:01.000", "answer": "Artist - Track" }
          ]
        }
        "#;

        let project = load_project_from_reader(json.as_bytes()).unwrap();
        assert_eq!(project.output.path, "render/out.mp4");
        assert_eq!(project.output.fps, Some(30));
        assert_eq!(project.clips.len(), 1);
        assert_eq!(project.clips[0].answer, "Artist - Track");
    }

    #[test]
    fn load_project_from_reader_rejects_invalid_json() {
        let json = r#"{ "output": { "path": "x.mp4" } }"#; // timings + clips manquants

        let err = load_project_from_reader(json.as_bytes()).unwrap_err();
        // On ne teste pas tout le message (ça peut changer), juste que c'est une erreur.
        assert!(err.to_string().contains("invalid JSON") || err.to_string().contains("missing field"));
    }
}

#[cfg(test)]
mod parsing_tests {
    use super::*;
    use crate::model::Project;

    fn parse(s: &str) -> anyhow::Result<Project> {
        load_project_from_reader(s.as_bytes())
    }

#[test]
fn fails_on_unknown_fields() {
    let json = r#"
    {
      "output": { "path": "render/out.mp4", "fps": 30, "unknown": 123 },
      "timings": { "guess_duration": "00:00:10.000", "reveal_duration": "00:00:05.000" },
      "clips": [
        { "video": "videos/a.mp4", "start": "00:00:01.000", "answer": "A" }
      ]
    }
    "#;

    let err = parse(json).unwrap_err();

    // `to_string()` peut n'afficher que le contexte ("invalid JSON").
    // `{:#}` inclut généralement toute la chaîne d'erreurs (causes).
    let full = format!("{:#}", err);
    assert!(full.contains("unknown field"), "error was:\n{full}");
}
}
