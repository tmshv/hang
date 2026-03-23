mod duration;
mod time;

use duration::parse_duration;
use time::parse_time;
use std::time::Duration;

fn parse_args() -> Result<Duration, String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Ok(Duration::from_secs(1));
    }

    let input = &args[1];
    if input.contains(':') {
        return parse_time(input);
    }

    parse_duration(input).map_err(|_| format!("invalid duration: '{}'", input))
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
