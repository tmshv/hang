use chrono::{Local, NaiveTime, TimeZone};
use std::time::Duration;

pub fn parse_time(input: &str) -> Result<Duration, String> {
    let target_time = NaiveTime::parse_from_str(input, "%H:%M:%S")
        .map_err(|_| format!("invalid time: '{}'", input))?;

    let now = Local::now();
    let target_naive = now.date_naive().and_time(target_time);
    let target_dt = match Local.from_local_datetime(&target_naive) {
        chrono::LocalResult::Single(dt) => dt,
        chrono::LocalResult::Ambiguous(dt1, dt2) => {
            if dt1 > now { dt1 } else { dt2 }
        }
        chrono::LocalResult::None => {
            return Err(format!("invalid time: '{}' does not exist in local time (DST gap)", input))
        }
    };

    let delta = target_dt.signed_duration_since(now);
    match delta.num_nanoseconds() {
        Some(ns) if ns > 0 => Ok(Duration::from_nanos(ns as u64)),
        _ => Ok(Duration::from_secs(0)),
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
