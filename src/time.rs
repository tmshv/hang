use std::time::{Duration, SystemTime};

pub fn parse_time(input: &str) -> Result<Duration, String> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
