// use std::error::Error;
use crate::error;
use crate::logger;
use crate::secgrep;
use std::fmt::Debug;
use std::path::PathBuf;
use structopt::StructOpt;
use walkdir::WalkDir;

static LOGGA: logger::Logga = logger::Logga {};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "secfind",
    version = "0.3.0",
    about = "grep for security-aware patterns like passphrases"
)]
pub struct SecFindCommand {
    /// Specifies the search directory to use
    #[structopt(name = "DIR", parse(from_os_str))]
    pub dirname: PathBuf,
}

impl SecFindCommand {
    pub fn run(&self) -> Result<i32, error::Error> {
        if !self.dirname.exists() {
            // return Err(error::ErrorType::Regular(error::ErrorKind::FileNotFound));
            let error_affected = self.dirname.to_str().unwrap_or("unknown file").to_string();
            let err = error::Error {
                error_type: error::ErrorType::Regular(error::ErrorKind::FileNotFound),
                affected: error_affected,
                suggestion: String::from(
                    "Verify if the file exists and you have specified the dirname correct.",
                ),
            };
            return Err(err);
        }

        let mut ret = 0;
        for entry in WalkDir::new(&self.dirname)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }
            let grep_command = secgrep::SecGrepCommand {
                filename: entry.path().to_path_buf(),
            };
            // ret += grep_command.run().unwrap();
            match grep_command.run() {
                Ok(n) => ret += n,
                Err(mut e) => {
                    e.set_affected(
                        entry
                            .path()
                            .to_path_buf()
                            .into_os_string()
                            .into_string()
                            .unwrap(),
                    );
                    LOGGA.error(&e);
                    return Err(e);
                }
            }
        }
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use crate::error;
    use crate::secfind;

    #[test]
    fn test_run_grep_command1() {
        use std::path::PathBuf;
        let grep_command = secfind::SecFindCommand {
            dirname: PathBuf::from(r"./tests"),
        };
        let result = grep_command.run().unwrap();
        assert_eq!((4), result);
    }

    #[test]
    fn test_run_grep_command_file_not_found() {
        use std::path::PathBuf;
        let grep_command = secfind::SecFindCommand {
            dirname: PathBuf::from(r"/file/does/not/exist/foobar"),
        };
        let _result = grep_command.run().unwrap_err();
        let err = error::ErrorType::Regular(error::ErrorKind::FileNotFound);
        assert!(matches!(err, _result));
    }
}
