// src/validate.rs
use anyhow::{Context, Result, bail};

use crate::model::Project;
use crate::timecode::parse_timecode_ms;

pub fn validate_project(p: &Project) -> Result<()> {
    if p.output.path.trim().is_empty() {
        bail!("output.path must not be empty");
    }

    // intro (optional)
    if let Some(intro) = p.intro.as_ref() {
        if intro.background.trim().is_empty() {
            bail!("intro.background must not be empty");
        }
        if intro.music.trim().is_empty() {
            bail!("intro.music must not be empty");
        }
        if intro.title.trim().is_empty() {
            bail!("intro.title must not be empty");
        }

        let intro_ms =
            parse_timecode_ms(intro.duration.trim()).context("intro.duration is invalid")?;
        if intro_ms == 0 {
            bail!("intro.duration must be > 0");
        }
    }

    // timings: parse + strictly > 0
    let guess_ms = parse_timecode_ms(p.timings.guess_duration.trim())
        .context("timings.guess_duration is invalid")?;
    let reveal_ms = parse_timecode_ms(p.timings.reveal_duration.trim())
        .context("timings.reveal_duration is invalid")?;

    if guess_ms == 0 {
        bail!("timings.guess_duration must be > 0");
    }
    if reveal_ms == 0 {
        bail!("timings.reveal_duration must be > 0");
    }

    // clips: at least one
    if p.clips.is_empty() {
        bail!("clips must not be empty");
    }

    for (i, c) in p.clips.iter().enumerate() {
        if c.video.trim().is_empty() {
            bail!("clips[{i}].video must not be empty");
        }
        if c.answer.trim().is_empty() {
            bail!("clips[{i}].answer must not be empty");
        }
        parse_timecode_ms(c.start.trim())
            .with_context(|| format!("clips[{i}].start is invalid"))?;
    }

    // Optional output validation (light, V1):
    if let Some(fps) = p.output.fps {
        if fps == 0 {
            bail!("output.fps must be > 0");
        }
    }

    if let Some(res) = p.output.resolution.as_deref() {
        let res = res.trim();
        if !is_resolution(res) {
            bail!("output.resolution must be formatted as WIDTHxHEIGHT (e.g. 1920x1080)");
        }
    }

    Ok(())
}

fn is_resolution(s: &str) -> bool {
    // Simple strict check: <digits>x<digits> and both > 0
    let Some((w, h)) = s.split_once('x') else {
        return false;
    };
    if w.is_empty() || h.is_empty() {
        return false;
    }
    if !w.chars().all(|c| c.is_ascii_digit()) || !h.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let Ok(w) = w.parse::<u32>() else {
        return false;
    };
    let Ok(h) = h.parse::<u32>() else {
        return false;
    };
    w > 0 && h > 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Clip, Intro, Output, Timings};

    fn base_project() -> Project {
        Project {
            intro: None,
            output: Output {
                path: "render/out.mp4".into(),
                resolution: Some("1920x1080".into()),
                fps: Some(30),
            },
            timings: Timings {
                guess_duration: "00:00:10.000".into(),
                reveal_duration: "00:00:05.000".into(),
            },
            clips: vec![Clip {
                video: "videos/a.mp4".into(),
                start: "00:00:01.000".into(),
                answer: "Artist - Track".into(),
            }],
        }
    }

    #[test]
    fn valid_project_passes() {
        let p = base_project();
        validate_project(&p).unwrap();
    }

    #[test]
    fn rejects_empty_clips() {
        let mut p = base_project();
        p.clips.clear();
        assert!(validate_project(&p).is_err());
    }

    #[test]
    fn rejects_empty_answer() {
        let mut p = base_project();
        p.clips[0].answer = "   ".into();
        assert!(validate_project(&p).is_err());
    }

    #[test]
    fn rejects_invalid_start_timecode() {
        let mut p = base_project();
        p.clips[0].start = "banana".into();
        assert!(validate_project(&p).is_err());
    }

    #[test]
    fn rejects_zero_durations() {
        let mut p = base_project();
        p.timings.guess_duration = "00:00:00.000".into();
        assert!(validate_project(&p).is_err());

        let mut p = base_project();
        p.timings.reveal_duration = "00:00:00.000".into();
        assert!(validate_project(&p).is_err());
    }

    #[test]
    fn rejects_bad_resolution() {
        let mut p = base_project();
        p.output.resolution = Some("1920-1080".into());
        assert!(validate_project(&p).is_err());
    }

    #[test]
    fn rejects_fps_zero() {
        let mut p = base_project();
        p.output.fps = Some(0);
        assert!(validate_project(&p).is_err());
    }

    #[test]
    fn intro_valid_passes() {
        let mut p = base_project();
        p.intro = Some(Intro {
            background: "assets/intro.png".into(),
            title: "Blind Test".into(),
            music: "assets/intro.mp3".into(),
            duration: "00:00:03.000".into(),
        });
        validate_project(&p).unwrap();
    }

    #[test]
    fn intro_rejects_zero_duration() {
        let mut p = base_project();
        p.intro = Some(Intro {
            background: "assets/intro.png".into(),
            title: "Blind Test".into(),
            music: "assets/intro.mp3".into(),
            duration: "00:00:00.000".into(),
        });
        assert!(validate_project(&p).is_err());
    }

    #[test]
    fn intro_rejects_empty_fields() {
        let mut p = base_project();
        p.intro = Some(Intro {
            background: "   ".into(),
            title: "".into(),
            music: " ".into(),
            duration: "00:00:03.000".into(),
        });
        assert!(validate_project(&p).is_err());
    }
}
