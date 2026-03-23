mod duration;
mod time;

use duration::parse_duration;
use time::parse_time;
use std::time::Duration;

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
