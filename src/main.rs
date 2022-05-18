// extern crate core;

use std::process;

use minigrep::Config;

fn main() {
    if let Err(e) = minigrep::run(Config::new()) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
