// src/lib.rs

pub mod model;

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

