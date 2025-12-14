// src/ffmpeg.rs

use crate::ffmpeg_command::CommandSpec;
use anyhow::{bail, Context, Result};
use std::process::{Command, Stdio};

/// Run the provided ffmpeg command spec.
///
/// - Captures stderr (ffmpeg writes progress/errors there)
/// - Returns an error if exit status != 0
pub fn run(spec: &CommandSpec) -> Result<()> {
    let mut cmd = Command::new(&spec.program);
    cmd.args(&spec.args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());

    let output = cmd
        .output()
        .with_context(|| format!("failed to spawn {}", spec.program))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "ffmpeg failed (exit code {:?}). stderr:\n{}",
            output.status.code(),
            stderr
        );
    }

    Ok(())
}

/// Format the command as a shell-like string for display/debugging.
/// (We do minimal quoting so spaces are readable.)
pub fn format_command(spec: &CommandSpec) -> String {
    let mut s = String::new();
    s.push_str(&spec.program);

    for arg in &spec.args {
        s.push(' ');
        s.push_str(&quote_arg(arg));
    }
    s
}

fn quote_arg(arg: &str) -> String {
    // Simple quoting: if it contains whitespace or quotes, wrap in double quotes and escape double quotes.
    if arg.chars().any(|c| c.is_whitespace() || c == '"' ) {
        let escaped = arg.replace('"', r#"\""#);
        format!("\"{escaped}\"")
    } else {
        arg.to_string()
    }
}

