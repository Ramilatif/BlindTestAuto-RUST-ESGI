// src/ffmpeg_command.rs

use crate::model::Project;
use crate::timecode::parse_timecode_ms;
use anyhow::{bail, Result};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSpec {
    pub program: String,      // "ffmpeg"
    pub args: Vec<String>,     // argv
}

/// Build a single-ffmpeg command (Option B) using `-filter_complex`.
///
/// Pipeline per clip:
/// - trim total (guess+reveal) from `start`
/// - split
/// - guess: black screen + countdown (MM:SS), audio kept
/// - reveal: video + answer overlay, audio kept
/// - concat guess+reveal
/// Then concat all clips into [vout][aout].
pub fn build_ffmpeg_command(p: &Project) -> Result<CommandSpec> {
    // Defaults (V1): if output params missing, pick deterministic values
    let (w, h) = parse_resolution(p.output.resolution.as_deref().unwrap_or("1920x1080"))?;
    let fps = p.output.fps.unwrap_or(30);

    let guess_ms = parse_timecode_ms(p.timings.guess_duration.trim())?;
    let reveal_ms = parse_timecode_ms(p.timings.reveal_duration.trim())?;
    if guess_ms == 0 || reveal_ms == 0 {
        bail!("guess_duration and reveal_duration must be > 0");
    }

    let guess_s = ms_to_seconds_f64(guess_ms);
    let reveal_s = ms_to_seconds_f64(reveal_ms);
    let total_s = guess_s + reveal_s;

    // Inputs: one -i per clip (even if file repeats; can dedupe later)
    let mut inputs: Vec<PathBuf> = Vec::with_capacity(p.clips.len());
    for c in &p.clips {
        inputs.push(PathBuf::from(c.video.trim()));
    }

    let filter_complex = build_filter_complex(p, &inputs, w, h, fps, guess_s, reveal_s, total_s)?;

    let mut args: Vec<String> = Vec::new();

    // Overwrite output by default in V1 (you can wire CLI --overwrite later)
    args.push("-y".into());

    // Inputs
    for input in &inputs {
        args.push("-i".into());
        args.push(input.to_string_lossy().to_string());
    }

    args.push("-filter_complex".into());
    args.push(filter_complex);

    // Output maps
    args.push("-map".into());
    args.push("[vout]".into());
    args.push("-map".into());
    args.push("[aout]".into());

    // Encoding (simple V1)
    args.push("-c:v".into());
    args.push("libx264".into());
    args.push("-pix_fmt".into());
    args.push("yuv420p".into());
    args.push("-c:a".into());
    args.push("aac".into());

    // Output path
    args.push(p.output.path.trim().to_string());

    Ok(CommandSpec {
        program: "ffmpeg".into(),
        args,
    })
}

fn build_filter_complex(
    p: &Project,
    inputs: &[PathBuf],
    w: u32,
    h: u32,
    fps: u32,
    guess_s: f64,
    reveal_s: f64,
    total_s: f64,
) -> Result<String> {
    let mut parts: Vec<String> = Vec::new();

    for (i, clip) in p.clips.iter().enumerate() {
        let start_ms = parse_timecode_ms(clip.start.trim())?;
        let start_s = ms_to_seconds_f64(start_ms);

        // Labels
        let v_all = format!("[v{i}all]");
        let a_all = format!("[a{i}all]");
        //let v_gsrc = format!("[v{i}gsrc]");
        //let v_rsrc = format!("[v{i}rsrc]");
        let a_gsrc = format!("[a{i}gsrc]");
        let a_rsrc = format!("[a{i}rsrc]");
        let v_g = format!("[v{i}g]");
        let a_g = format!("[a{i}g]");
        let v_r = format!("[v{i}r]");
        let a_r = format!("[a{i}r]");
        let v_i = format!("[v{i}]");
        let a_i = format!("[a{i}]");

        // 1) Trim + normalize video
        // Use trim start/duration and reset timestamps
        parts.push(format!(
            "[{i}:v]trim=start={start_s:.3}:duration={total_s:.3},setpts=PTS-STARTPTS,\
scale={w}:{h},fps={fps},setsar=1{v_all}",
        ));

        // 2) Trim audio
        parts.push(format!(
            "[{i}:a]atrim=start={start_s:.3}:duration={total_s:.3},asetpts=PTS-STARTPTS{a_all}",
        ));

        // 3) Split
       // parts.push(format!("{v_all}split=2{v_gsrc}{v_rsrc}"));
        parts.push(format!("{a_all}asplit=2{a_gsrc}{a_rsrc}"));

        // 4) Guess video: black screen + countdown MM:SS
        // Countdown expression (MM:SS):
        // minutes = floor(max(0, GUESS - t)/60)
        // seconds = mod(floor(max(0, GUESS - t)), 60)
        let countdown_text = format!(
            "%{{eif\\:floor(max(0\\,{guess_s:.3}-t)/60)\\:02d}}\\:%{{eif\\:mod(floor(max(0\\,{guess_s:.3}-t))\\,60)\\:02d}}"
        );

        parts.push(format!(
            "color=c=black:s={w}x{h}:r={fps}:d={guess_s:.3},\
drawtext=text='{countdown_text}':x=(w-text_w)/2:y=(h-text_h)/2:fontsize=96:fontcolor=white:borderw=4{v_g}"
        ));

        // Guess audio: first segment [0, guess]
        parts.push(format!(
            "{a_gsrc}atrim=0:{guess_s:.3},asetpts=PTS-STARTPTS{a_g}"
        ));

        // 5) Reveal video: trim [guess, guess+reveal] + answer overlay
        let answer = escape_drawtext_text(clip.answer.trim());
        parts.push(format!(
            "{v_all}trim=start={guess_s:.3}:duration={reveal_s:.3},setpts=PTS-STARTPTS,\
drawtext=text='{answer}':x=(w-text_w)/2:y=h-(text_h*2):fontsize=48:fontcolor=white:borderw=3{v_r}"
        ));

        // Reveal audio: trim [guess, guess+reveal]
        parts.push(format!(
            "{a_rsrc}atrim=start={guess_s:.3}:duration={reveal_s:.3},asetpts=PTS-STARTPTS{a_r}"
        ));

        // 6) Concat guess+reveal into one segment per clip
        parts.push(format!(
            "{v_g}{a_g}{v_r}{a_r}concat=n=2:v=1:a=1{v_i}{a_i}"
        ));
    }

    // Final concat of all clips
    let mut concat_in = String::new();
    for i in 0..inputs.len() {
        concat_in.push_str(&format!("[v{i}][a{i}]"));
    }
    parts.push(format!("{concat_in}concat=n={}:v=1:a=1[vout][aout]", inputs.len()));

    Ok(parts.join(";"))
}

/// Escape user text for ffmpeg drawtext inside single quotes.
///
/// Minimal safe set for our usage:
/// - backslash -> \\
/// - single quote -> \'
/// (You can extend later for ':' etc if needed.)
fn escape_drawtext_text(s: &str) -> String {
    s.replace('\\', r"\\").replace('\'', r"\'")
}

fn parse_resolution(res: &str) -> Result<(u32, u32)> {
    let res = res.trim();
    let Some((w, h)) = res.split_once('x') else {
        bail!("invalid resolution '{res}', expected WIDTHxHEIGHT");
    };
    let w: u32 = w.parse()?;
    let h: u32 = h.parse()?;
    if w == 0 || h == 0 {
        bail!("resolution must be > 0");
    }
    Ok((w, h))
}

fn ms_to_seconds_f64(ms: u64) -> f64 {
    (ms as f64) / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Clip, Output, Project, Timings};

    fn project_one_clip() -> Project {
        Project {
            output: Output {
                path: "render/out.mp4".into(),
                resolution: Some("1280x720".into()),
                fps: Some(30),
            },
            timings: Timings {
                guess_duration: "00:00:10.000".into(),
                reveal_duration: "00:00:05.000".into(),
            },
            clips: vec![Clip {
                video: "videos/a.mp4".into(),
                start: "00:00:01.000".into(),
                answer: "Guns N' Roses - Live".into(),
            }],
        }
    }

    #[test]
    fn builds_one_ffmpeg_command() {
        let p = project_one_clip();
        let spec = build_ffmpeg_command(&p).unwrap();

        assert_eq!(spec.program, "ffmpeg");
        assert!(spec.args.iter().any(|a| a == "-filter_complex"));
        assert!(spec.args.iter().any(|a| a == "render/out.mp4"));

        let fc = spec
            .args
            .iter()
            .skip_while(|a| *a != "-filter_complex")
            .nth(1)
            .unwrap()
            .clone();

        // Has black guess screen with countdown
        assert!(fc.contains("color=c=black"));
        assert!(fc.contains("drawtext=text="));
        assert!(fc.contains("max(0\\,10.000-t)"), "filter_complex was:\n{fc}");

        // Has answer overlay in reveal (with escaped quote)
        assert!(fc.contains("Guns N\\' Roses - Live"), "filter_complex was:\n{fc}");

        // Final concat outputs
        assert!(fc.contains("[vout][aout]"));
        assert!(fc.contains("concat=n=1:v=1:a=1[vout][aout]"));
    }

    #[test]
    fn builds_concat_for_two_clips() {
        let mut p = project_one_clip();
        p.clips.push(Clip {
            video: "videos/b.mp4".into(),
            start: "00:00:02.000".into(),
            answer: "Daft Punk - One More Time".into(),
        });

        let spec = build_ffmpeg_command(&p).unwrap();
        let fc = spec
            .args
            .iter()
            .skip_while(|a| *a != "-filter_complex")
            .nth(1)
            .unwrap()
            .clone();

        // Should contain both clip labels and concat n=2
        assert!(fc.contains("[v0][a0][v1][a1]concat=n=2:v=1:a=1[vout][aout]"), "filter_complex was:\n{fc}");
    }
}

