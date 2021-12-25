// use std::error::Error;
use crate::error;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
pub mod search;

#[derive(Debug, StructOpt)]
#[structopt(name = "egrep", version = "0.2.0", about = "egrep clone")]
pub struct EGrepCommand {
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

impl EGrepCommand {
    pub fn run(&self) -> Result<(), error::Error> {
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

        let contents = fs::read_to_string(&self.filename)?;
        /*
        let contents = fs::read_to_string(&self.filename);
        let mut contents = match contents {
            Ok(contents) => contents,
            Err(e) => return error::ErrorType::Io(e),
        };
        */

        let results = if self.case_insensitive {
            search::search_case_insensitive(&self.pattern, &contents)
        } else {
            search::search(&self.pattern, &contents)
        };

        for line in results {
            println!("{}", line);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::egrep;
    use crate::error;

    #[test]
    fn test_run_grep_command() {
        use std::path::PathBuf;
        let grep_command = egrep::EGrepCommand {
            pattern: String::from("Hello, world!"),
            filename: PathBuf::from(r"./tests/test.file"),
            case_insensitive: false,
        };
        let result = grep_command.run().unwrap();
        assert_eq!((), result);
    }

    #[test]
    fn test_run_grep_command_file_not_found() {
        use std::path::PathBuf;
        let grep_command = egrep::EGrepCommand {
            pattern: String::from("Hello, world!"),
            filename: PathBuf::from(r"/file/does/not/exist/foobar"),
            case_insensitive: false,
        };
        let _result = grep_command.run().unwrap_err();
        let err = error::ErrorType::Regular(error::ErrorKind::FileNotFound);
        assert!(matches!(err, _result));
    }
}
