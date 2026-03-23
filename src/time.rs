use chrono::{Local, NaiveTime, Timelike};
use std::time::Duration;

pub fn parse_time(input: &str) -> Result<Duration, String> {
    let target = NaiveTime::parse_from_str(input, "%H:%M:%S")
        .map_err(|_| format!("invalid time: '{}'", input))?;

    let now = Local::now().time();
    let target_ns = target.num_seconds_from_midnight() as i64 * 1_000_000_000
        + target.nanosecond() as i64;
    let now_ns = now.num_seconds_from_midnight() as i64 * 1_000_000_000
        + now.nanosecond() as i64;
    let delta_ns = target_ns - now_ns;

    if delta_ns > 0 {
        Ok(Duration::from_nanos(delta_ns as u64))
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
        assert_eq!(parse_time("25:00:00"), Err("invalid time: '25:00:00'".to_string()));
        assert_eq!(parse_time("12:60:00"), Err("invalid time: '12:60:00'".to_string()));
        assert_eq!(parse_time("12:00:61"), Err("invalid time: '12:00:61'".to_string()));
        assert_eq!(parse_time("notadate"), Err("invalid time: 'notadate'".to_string()));
    }

    #[test]
    fn test_parse_time_past_time_returns_zero() {
        // 00:00:00 is always in the past (or right now), should return 0
        assert_eq!(parse_time("00:00:00"), Ok(Duration::from_secs(0)));
    }

    #[test]
    fn test_parse_time_invalid_input() {
        assert_eq!(parse_time("not a date"), Err("invalid time: 'not a date'".to_string()));
        assert_eq!(parse_time(""), Err("invalid time: ''".to_string()));
        assert_eq!(parse_time("2022-01-01T00:00:00Z"), Err("invalid time: '2022-01-01T00:00:00Z'".to_string()));
    }
}
