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
