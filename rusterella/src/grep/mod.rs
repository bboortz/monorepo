pub mod config;
use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "grep", version = "0.1.0", about = "grep clone")]
pub struct GrepCommand {
    /// activate case case_insensitive grep
    #[structopt(short = "i", long = "case_insensitive")]
    pub case_insensitive: bool,

    /// Specifies the pattern to search for
    #[structopt(name = "PATTERN")]
    pub pattern: String,

    /// Specifies the input file to use
    #[structopt(name = "FILE", parse(from_os_str))]
    pub filename: PathBuf,
}

#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub filename: PathBuf,
    pub case_insensitive: bool,
}

impl Config {}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_insensitive {
        search_case_insensitive(&config.pattern, &contents)
    } else {
        search(&config.pattern, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    if !pattern.is_empty() {
        for line in contents.lines() {
            if line.contains(pattern) {
                results.push(line);
            }
        }
    }

    results
}

pub fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let pattern = pattern.to_lowercase();

    if !pattern.is_empty() {
        for line in contents.lines() {
            if line.to_lowercase().contains(&pattern) {
                results.push(line);
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let pattern = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(pattern, contents));
    }

    #[test]
    fn no_result() {
        let pattern = "FOO";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let mut results = vec![""];
        results.pop();

        assert_eq!(results, search(pattern, contents));
    }

    #[test]
    fn no_result2() {
        let pattern = "";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
        let mut results = vec![""];
        results.pop();

        assert_eq!(results, search(pattern, contents));
    }

    #[test]
    fn case_insensitive() {
        let pattern = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(pattern, contents));
    }

    #[test]
    fn case_insensitive() {
        let pattern = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(pattern, contents)
        );
    }
}
