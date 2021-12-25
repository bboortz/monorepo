/*
extern crate walkdir;

use std::fs::File;
use std::io;
use std::io::Read;
use walkdir::WalkDir;

fn main() -> Result<(), io::Error> {
    let nul: u8 = 0;
    let mut bytes_count: i32;
    let mut buffer = Vec::new();

    for entry in WalkDir::new("./").into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let mut file = File::open(path)?;

        bytes_count = 0;
        buffer.clear();
        file.read_to_end(&mut buffer)?;

        for b in buffer.iter() {
            if b == &nul {
                println!("{} bytes: {} binary file", bytes_count, path.display());
                break;
            }

            bytes_count += 1;
        }

        println!("{} bytes: {}", bytes_count, path.display())
    }
    Ok(())
}
*/

////////////

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
    fn test_one_result() {
        let pattern = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(pattern, contents));
    }

    #[test]
    fn test_no_result() {
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
    fn test_no_result2() {
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
    fn test_case_insensitive() {
        let pattern = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(pattern, contents));
    }

    #[test]
    fn test_case_insensitive2() {
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
