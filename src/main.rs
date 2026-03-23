mod duration;
mod time;

use duration::parse_duration;
use std::time::Duration;
use time::parse_time;

fn is_time_format(s: &str) -> bool {
    let parts: Vec<&str> = s.split(':').collect();
    (parts.len() == 2 || parts.len() == 3)
        && parts
            .iter()
            .all(|p| matches!(p.as_bytes(), [b'0'..=b'9', b'0'..=b'9']))
}

fn parse_args() -> Result<Duration, String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Ok(Duration::from_secs(1));
    }

    let input = &args[1];
    if is_time_format(input) {
        let normalized = if input.len() == 5 {
            format!("{}:00", input)
        } else {
            input.clone()
        };
        return parse_time(&normalized);
    }

    parse_duration(input).map_err(|_| format!("invalid duration: '{}'", input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_time_format_hh_mm_ss() {
        assert!(is_time_format("23:43:00"));
        assert!(is_time_format("00:00:00"));
    }

    #[test]
    fn test_is_time_format_hh_mm() {
        assert!(is_time_format("23:43"));
        assert!(is_time_format("00:00"));
    }

    #[test]
    fn test_is_time_format_invalid() {
        assert!(!is_time_format("23:4"));
        assert!(!is_time_format("2:43"));
        assert!(!is_time_format("5s"));
        assert!(!is_time_format("23:43:0"));
    }
}

fn main() {
    match parse_args() {
        Ok(dur) => std::thread::sleep(dur),
        Err(msg) => {
            eprintln!("hang: {}", msg);
            std::process::exit(1);
        }
    }
}
