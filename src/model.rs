// src/model.rs
use serde::{Deserialize, Serialize};

/// Root JSON document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub output: Output,
    pub timings: Timings,
    pub clips: Vec<Clip>,
}

/// Output rendering parameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Output {
    /// Path of the final rendered video file
    pub path: String,

    /// Optional output resolution like "1920x1080"
    #[serde(default)]
    pub resolution: Option<String>,

    /// Optional output frames per second
    #[serde(default)]
    pub fps: Option<u32>,
}

/// Global timings applied to every clip
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Timings {
    /// Duration of the "guess" phase
    pub guess_duration: String,

    /// Duration of the "reveal" phase
    pub reveal_duration: String,
}

/// One blindtest item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Clip {
    /// Source video file path
    pub video: String,

    /// Start timecode in the source video
    pub start: String,

    /// Answer text displayed during the reveal phase
    pub answer: String,
}
