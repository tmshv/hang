use chrono::{Local, NaiveTime, Timelike};
use std::time::Duration;

pub fn parse_time(input: &str) -> Result<Duration, String> {
    let target = NaiveTime::parse_from_str(input, "%H:%M:%S")
        .map_err(|_| format!("invalid time: '{}'", input))?;

    let now = Local::now().time();
    let target_secs = target.num_seconds_from_midnight() as i64;
    let now_secs = now.num_seconds_from_midnight() as i64;
    let delta = target_secs - now_secs;

    if delta > 0 {
        Ok(Duration::from_secs(delta as u64))
    } else {
        Ok(Duration::from_secs(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_time_hms_format() {
        let result = parse_time("23:59:59");
        assert!(result.is_ok());

        let result = parse_time("00:00:00");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_time_invalid_hms() {
        assert!(parse_time("25:00:00").is_err()); // invalid hour
        assert!(parse_time("12:60:00").is_err()); // invalid minute
        assert!(parse_time("12:00:61").is_err()); // invalid second (60 is valid leap second in chrono)
        assert!(parse_time("notadate").is_err());
    }

    #[test]
    fn test_parse_time_past_time_returns_zero() {
        // 00:00:00 is always in the past (or right now), should return 0
        assert_eq!(parse_time("00:00:00"), Ok(Duration::from_secs(0)));
    }

    #[test]
    fn test_parse_time_invalid_input() {
        assert!(parse_time("not a date").is_err());
        assert!(parse_time("").is_err());
        assert!(parse_time("2022-01-01T00:00:00Z").is_err());
    }
}
