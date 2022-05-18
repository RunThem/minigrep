use std::error::Error;
use std::fs;

use clap::Parser;

// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(
    author = "RunThem <iccy.fun@outlook.com>",
    version = "0.1.2",
    about = "mini grep",
    long_about = "A small grep implementation written in rust"
)]
pub struct Config {
    #[clap(short)]
    /// is it case sensitive?
    pub case_sensitive: bool,

    /// string to query
    pub query: String,
    /// query in that file
    pub filename: String,
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}

impl Config {
    pub fn new() -> Config {
        Config::parse()
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    for line in search(config.case_sensitive, &config.query, &contents) {
        println!("{}", line)
    }

    Ok(())
}

pub fn search<'a>(case_sensitive: bool, query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|&line| match case_sensitive {
            true => line.to_lowercase().contains(&query.to_lowercase()),
            false => line.contains(query),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["Rust:"], search(false, query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(true, query, contents));
    }
}
