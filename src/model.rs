// src/model.rs
use serde::{Deserialize, Serialize};

/// Root JSON document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Project {
    pub output: Output,
    pub timings: Timings,
    pub clips: Vec<Clip>,
}

/// Output rendering parameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Output {
    /// Path of the final rendered video file (e.g. "render/blindtest.mp4")
    pub path: String,

    /// Optional output resolution like "1920x1080"
    #[serde(default)]
    pub resolution: Option<String>,

    /// Optional output frames per second (e.g. 30)
    #[serde(default)]
    pub fps: Option<u32>,
}

/// Global timings applied to every clip
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Timings {
    /// Duration of the "guess" phase: audio only, video hidden (e.g. "00:00:10.000")
    pub guess_duration: String,

    /// Duration of the "reveal" phase: video + answer overlay (e.g. "00:00:05.000")
    pub reveal_duration: String,
}

/// One blindtest item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Clip {
    /// Source video file path
    pub video: String,

    /// Start timecode in the source video (e.g. "00:01:23.500")
    pub start: String,

    /// Answer text displayed during the reveal phase
    pub answer: String,
}

