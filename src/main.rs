use regex::Regex;
use std::time::{Duration, SystemTime};

#[derive(Debug, PartialEq)]
struct DurationError;

fn parse_duration(s: &str) -> Result<Duration, DurationError> {
    let re = Regex::new(r"(\d+)(ms|s|m|h)?$").unwrap();
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

fn parse_time(input: &str) -> Result<Duration, String> {
    match dateparser::parse(input) {
        Ok(parsed) => {
            let now = SystemTime::now();
            let current_time = now
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|_| "SystemTime before UNIX EPOCH!".to_string())?
                .as_micros();
            let target = parsed.timestamp_micros() as u128;
            if target > current_time {
                let delta = target - current_time;
                return Ok(Duration::from_micros(delta as u64));
            }
            Ok(Duration::from_micros(0))
        }
        Err(_) => Err("Failed to parse".into()),
    }
}

fn parse_args() -> Duration {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Duration::from_secs(1); // Sleep for 1 second if no arguments passed
    }

    let input = &args[1];
    if input.contains(":") {
        return parse_time(input).unwrap_or(Duration::from_nanos(0));
    }

    match parse_duration(input) {
        Ok(dur) => dur,
        Err(_) => Duration::from_nanos(0),
    }
}

fn main() {
    // Parse duration value from cli args
    let dur = parse_args();

    // print!("duration {:?}", dur);

    // Sleep for the specified number of seconds
    std::thread::sleep(dur);

    // Exit the program
    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    // parse_duration tests

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
    fn test_parse_duration_ns_not_supported() {
        // Known bug: regex pattern does not include "ns" as a unit,
        // so "100ns" is rejected even though the match arm exists.
        assert_eq!(parse_duration("100ns"), Err(DurationError));
    }

    #[test]
    fn test_parse_duration_no_start_anchor() {
        // Known behavior: regex lacks ^ anchor, so leading junk is accepted
        // as long as the string ends with a valid pattern.
        assert_eq!(parse_duration("abc123s"), Ok(Duration::from_secs(123)));
        assert_eq!(parse_duration("xyz500ms"), Ok(Duration::from_millis(500)));
    }

    #[test]
    fn test_parse_duration_space_in_input() {
        // Space between number and unit breaks regex match
        assert_eq!(parse_duration("5 s"), Err(DurationError));
        // Leading space still matches because regex has no ^ anchor
        assert_eq!(parse_duration(" 5s"), Ok(Duration::from_secs(5)));
    }

    #[test]
    fn test_parse_duration_large_value() {
        assert_eq!(
            parse_duration("1000000ms"),
            Ok(Duration::from_millis(1_000_000))
        );
    }

    // parse_time tests

    #[test]
    fn test_parse_time_past_dates() {
        assert_eq!(
            parse_time("2022-01-01T00:00:00Z"),
            Ok(Duration::from_micros(0))
        );
        assert_eq!(
            parse_time("2000-06-15T12:00:00Z"),
            Ok(Duration::from_micros(0))
        );
    }

    #[test]
    fn test_parse_time_epoch() {
        assert_eq!(
            parse_time("1970-01-01T00:00:00Z"),
            Ok(Duration::from_micros(0))
        );
    }

    #[test]
    fn test_parse_time_invalid_input() {
        assert_eq!(parse_time("not a date"), Err("Failed to parse".to_string()));
        assert_eq!(parse_time(""), Err("Failed to parse".to_string()));
    }

    #[test]
    fn test_parse_time_future_date() {
        let result = parse_time("2999-01-01T00:00:00Z");
        assert!(result.is_ok());
        assert!(result.unwrap() > Duration::from_secs(0));
    }
}
