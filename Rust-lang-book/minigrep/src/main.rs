use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Collect arguments
    let args: Vec<String> = env::args().collect();

    // Build Config, handle errors
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Run the program, handle errors
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
