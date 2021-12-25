use crate::error;
use crate::lib::search_regex;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "secgrep",
    version = "0.3.0",
    about = "grep for security-aware patterns like passphrases"
)]
pub struct SecGrepCommand {
    /// Specifies the input file to use
    #[structopt(name = "FILE", parse(from_os_str))]
    pub filename: PathBuf,
}

impl SecGrepCommand {
    pub fn run(&self) -> Result<i32, error::Error> {
        let pattern = r"apikey|passphrase|password|secret";
        if !self.filename.exists() {
            // return Err(error::ErrorType::Regular(error::ErrorKind::FileNotFound));
            let error_affected = self.filename.to_str().unwrap_or("unknown file").to_string();
            let err = error::Error {
                error_type: error::ErrorType::Regular(error::ErrorKind::FileNotFound),
                affected: error_affected,
                suggestion: String::from(
                    "Verify if the file exists and you have specified the filename correct.",
                ),
            };
            return Err(err);
        }

        //let contents = fs::read_to_string(&self.filename)?;
        let contents = match fs::read_to_string(&self.filename) {
            Ok(c) => c,
            Err(_) => {
                let buf = fs::read(self.filename.as_path())?;
                String::from_utf8_lossy(&buf).to_string()
            }
        };
        //
        let mut ret = 0;
        let results = search_regex::search_case_insensitive(pattern, &contents);

        if !results.is_empty() {
            println!("*** {:?}", self.filename);
            for line in results {
                ret += 1;
                println!("{}", line);
            }
        }

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use crate::error;
    use crate::secgrep;

    #[test]
    fn test_run_grep_command1() {
        use std::path::PathBuf;
        let grep_command = secgrep::SecGrepCommand {
            filename: PathBuf::from(r"./tests/test.file"),
        };
        let result = grep_command.run().unwrap();
        assert_eq!((4), result);
    }

    #[test]
    fn test_run_grep_command_file_not_found() {
        use std::path::PathBuf;
        let grep_command = secgrep::SecGrepCommand {
            filename: PathBuf::from(r"/file/does/not/exist/foobar"),
        };
        let _result = grep_command.run().unwrap_err();
        let err = error::ErrorType::Regular(error::ErrorKind::FileNotFound);
        assert!(matches!(err, _result));
    }
}
