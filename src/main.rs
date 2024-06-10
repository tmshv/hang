use regex::Regex;
use std::time::{Duration, SystemTime};

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
