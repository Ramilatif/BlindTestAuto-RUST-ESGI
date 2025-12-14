// src/timecode.rs
use anyhow::{Context, Result, bail};

/// Parse a timecode formatted as `HH:MM:SS.mmm` into milliseconds.
///
/// Examples:
/// - "00:00:01.000" -> 1000
/// - "01:02:03.004" -> 3723004
pub fn parse_timecode_ms(s: &str) -> Result<u64> {
    // Strict format: exactly 12 chars "HH:MM:SS.mmm"
    // Example: 0 1 : 3 4 : 6 7 . 9 10 11
    if s.len() != 12 {
        bail!("invalid timecode length (expected HH:MM:SS.mmm): '{s}'");
    }
    let bytes = s.as_bytes();
    if bytes[2] != b':' || bytes[5] != b':' || bytes[8] != b'.' {
        bail!("invalid timecode separators (expected HH:MM:SS.mmm): '{s}'");
    }

    let hh =
        parse_2_digits(&s[0..2]).with_context(|| format!("invalid hours in timecode '{s}'"))?;
    let mm =
        parse_2_digits(&s[3..5]).with_context(|| format!("invalid minutes in timecode '{s}'"))?;
    let ss =
        parse_2_digits(&s[6..8]).with_context(|| format!("invalid seconds in timecode '{s}'"))?;
    let mmm = parse_3_digits(&s[9..12])
        .with_context(|| format!("invalid milliseconds in timecode '{s}'"))?;

    if mm > 59 {
        bail!("minutes out of range (0..59) in timecode '{s}'");
    }
    if ss > 59 {
        bail!("seconds out of range (0..59) in timecode '{s}'");
    }
    // mmm is 0..999 by construction (3 digits)

    let total_ms =
        (hh as u64) * 3_600_000 + (mm as u64) * 60_000 + (ss as u64) * 1_000 + (mmm as u64);

    Ok(total_ms)
}

fn parse_2_digits(s: &str) -> Result<u16> {
    if s.len() != 2 || !s.chars().all(|c| c.is_ascii_digit()) {
        bail!("expected 2 digits, got '{s}'");
    }
    Ok(s.parse::<u16>()?)
}

fn parse_3_digits(s: &str) -> Result<u16> {
    if s.len() != 3 || !s.chars().all(|c| c.is_ascii_digit()) {
        bail!("expected 3 digits, got '{s}'");
    }
    Ok(s.parse::<u16>()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_timecodes() {
        assert_eq!(parse_timecode_ms("00:00:00.000").unwrap(), 0);
        assert_eq!(parse_timecode_ms("00:00:01.000").unwrap(), 1_000);
        assert_eq!(parse_timecode_ms("00:01:00.000").unwrap(), 60_000);
        assert_eq!(parse_timecode_ms("01:00:00.000").unwrap(), 3_600_000);
        assert_eq!(parse_timecode_ms("01:02:03.004").unwrap(), 3_723_004);
        assert_eq!(parse_timecode_ms("99:59:59.999").unwrap(), 359_999_999);
    }

    #[test]
    fn rejects_bad_length() {
        assert!(parse_timecode_ms("0:00:01.000").is_err());
        assert!(parse_timecode_ms("00:00:01.00").is_err());
        assert!(parse_timecode_ms("").is_err());
    }

    #[test]
    fn rejects_bad_separators() {
        assert!(parse_timecode_ms("00-00:01.000").is_err());
        assert!(parse_timecode_ms("00:00-01.000").is_err());
        assert!(parse_timecode_ms("00:00:01,000").is_err());
    }

    #[test]
    fn rejects_non_digits() {
        assert!(parse_timecode_ms("aa:00:01.000").is_err());
        assert!(parse_timecode_ms("00:bb:01.000").is_err());
        assert!(parse_timecode_ms("00:00:cc.000").is_err());
        assert!(parse_timecode_ms("00:00:01.xxx").is_err());
    }

    #[test]
    fn rejects_out_of_range_minutes_seconds() {
        assert!(parse_timecode_ms("00:60:00.000").is_err());
        assert!(parse_timecode_ms("00:00:60.000").is_err());
    }
}
