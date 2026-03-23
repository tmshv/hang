use regex::Regex;
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub struct DurationError;

pub fn parse_duration(s: &str) -> Result<Duration, DurationError> {
    let re = Regex::new(r"^(\d+)(ns|ms|s|m|h)?$").unwrap();
    match re.captures(s) {
        Some(caps) => {
            let value = caps.get(1).map_or("", |m| m.as_str());
            let num = value.parse::<u64>().unwrap();
            let unit = &caps.get(2).map_or("", |m| m.as_str()).to_lowercase();
            let duration = match unit.as_str() {
                "ns" => Duration::from_nanos(num),
                "ms" => Duration::from_millis(num),
                "s" => Duration::from_secs(num),
                "m" => Duration::from_secs(num * 60),
                "h" => Duration::from_secs(num * 3600),
                _ => Duration::from_millis(num),
                // _ => return Err(DurationError),
            };
            Ok(duration)
        }
        None => Err(DurationError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration_each_unit() {
        assert_eq!(parse_duration("100ms"), Ok(Duration::from_millis(100)));
        assert_eq!(parse_duration("5s"), Ok(Duration::from_secs(5)));
        assert_eq!(parse_duration("10m"), Ok(Duration::from_secs(600)));
        assert_eq!(parse_duration("2h"), Ok(Duration::from_secs(7200)));
    }

    #[test]
    fn test_parse_duration_no_unit_defaults_to_ms() {
        assert_eq!(parse_duration("500"), Ok(Duration::from_millis(500)));
        assert_eq!(parse_duration("0"), Ok(Duration::from_millis(0)));
        assert_eq!(parse_duration("1"), Ok(Duration::from_millis(1)));
    }

    #[test]
    fn test_parse_duration_zero_with_units() {
        assert_eq!(parse_duration("0s"), Ok(Duration::from_secs(0)));
        assert_eq!(parse_duration("0ms"), Ok(Duration::from_millis(0)));
        assert_eq!(parse_duration("0m"), Ok(Duration::from_secs(0)));
        assert_eq!(parse_duration("0h"), Ok(Duration::from_secs(0)));
    }

    #[test]
    fn test_parse_duration_single_digit_with_units() {
        assert_eq!(parse_duration("1s"), Ok(Duration::from_secs(1)));
        assert_eq!(parse_duration("1m"), Ok(Duration::from_secs(60)));
        assert_eq!(parse_duration("1h"), Ok(Duration::from_secs(3600)));
    }

    #[test]
    fn test_parse_duration_invalid_strings() {
        assert_eq!(parse_duration(""), Err(DurationError));
        assert_eq!(parse_duration("invalid"), Err(DurationError));
        assert_eq!(parse_duration("abc"), Err(DurationError));
        assert_eq!(parse_duration("ms"), Err(DurationError));
        assert_eq!(parse_duration("s"), Err(DurationError));
    }

    #[test]
    fn test_parse_duration_ns() {
        assert_eq!(parse_duration("100ns"), Ok(Duration::from_nanos(100)));
        assert_eq!(parse_duration("0ns"), Ok(Duration::from_nanos(0)));
    }

    #[test]
    fn test_parse_duration_rejects_leading_junk() {
        assert_eq!(parse_duration("abc123s"), Err(DurationError));
        assert_eq!(parse_duration("xyz500ms"), Err(DurationError));
    }

    #[test]
    fn test_parse_duration_space_in_input() {
        // Space between number and unit breaks regex match
        assert_eq!(parse_duration("5 s"), Err(DurationError));
        assert_eq!(parse_duration(" 5s"), Err(DurationError));
    }

    #[test]
    fn test_parse_duration_large_value() {
        assert_eq!(
            parse_duration("1000000ms"),
            Ok(Duration::from_millis(1_000_000))
        );
    }
}
