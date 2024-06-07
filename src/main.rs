use std::thread;
use std::time::Duration;

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ./hang <seconds>");
        return;
    }

    let duration_str = &args[1];
    match duration_str.parse::<u64>() {
        Ok(seconds) => {
            // Sleep for the specified number of seconds
            thread::sleep(Duration::from_secs(seconds));
            println!("Goodbye!");
        }
        Err(_) => {
            eprintln!("Invalid duration: {}", duration_str);
            return;
        }
    }

    // Exit the program
    std::process::exit(0);
}
