// src/model.rs
use serde::{Deserialize, Serialize};

/// Root JSON document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Project {
    /// Optional intro shown before the blindtest starts
    #[serde(default)]
    pub intro: Option<Intro>,

    pub output: Output,
    pub timings: Timings,
    pub clips: Vec<Clip>,
}

/// Optional intro section
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Intro {
    /// Background image path (png/jpg)
    pub background: String,

    /// Title displayed on the intro screen
    pub title: String,

    /// Music file played during intro (mp3/wav/...)
    pub music: String,

    /// Intro duration (HH:MM:SS.mmm)
    pub duration: String,
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

