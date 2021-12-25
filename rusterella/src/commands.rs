use crate::commands::CommandsFassade::EGrepCommand;
use crate::commands::CommandsFassade::GrepCommand;
use crate::commands::CommandsFassade::SecFindCommand;
use crate::commands::CommandsFassade::SecGrepCommand;
use crate::egrep;
use crate::error;
use crate::grep;
use crate::secfind;
use crate::secgrep;
use std::fmt::Debug;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rusterella",
    version = "0.4.0",
    about = "A single binary written in rust than combines several unix utilities. A busybox clone in rust.",
    author = "Benjamin Boortz <benjamn dot boortz at mailbox dot org>"
)]
#[allow(clippy::enum_variant_names)]
pub enum CommandsFassade {
    #[structopt(name = "grep", version = "0.1.0", about = "grep clone")]
    GrepCommand(grep::GrepCommand),
    #[structopt(name = "egrep", version = "0.2.0", about = "egrep clone")]
    EGrepCommand(egrep::EGrepCommand),
    #[structopt(
        name = "secgrep",
        version = "0.3.0",
        about = "grep for security-aware patterns like passphrases"
    )]
    SecGrepCommand(secgrep::SecGrepCommand),
    #[structopt(
        name = "secfind",
        version = "0.4.0",
        about = "find for security-aware patterns like passphrases"
    )]
    SecFindCommand(secfind::SecFindCommand),
}

impl CommandsFassade {
    #[allow(dead_code)]
    pub fn print(&self) {
        match self {
            GrepCommand(grep_struct) => println!("GrepCommand: {:?}", grep_struct),
            EGrepCommand(grep_struct) => println!("EGrepCommand: {:?}", grep_struct),
            SecGrepCommand(grep_struct) => println!("SecGrepCommand: {:?}", grep_struct),
            SecFindCommand(grep_struct) => println!("SecFindCommand: {:?}", grep_struct),
        }
    }

    /*
    pub fn run(&self) -> Result<(), Box<error::ErrorType>> {
    */
    pub fn run(&self) -> Result<i32, error::Error> {
        Ok(match self {
            GrepCommand(grep_command) => {
                grep_command.run()?;
                0
            }
            EGrepCommand(egrep_command) => {
                egrep_command.run()?;
                0
            }
            SecGrepCommand(secgrep_command) => secgrep_command.run()?,
            SecFindCommand(secfind_command) => secfind_command.run()?,
        })
    }
}

/*
pub trait Command {
    fn run(&self) -> Result<i32, error::Error>;
}
*/

#[cfg(test)]
mod tests {
    use crate::commands;
    use crate::error;
    use crate::grep;
    use std::path::PathBuf;

    #[test]
    fn test_commandsfassade_grep_ok() {
        let grep_command = grep::GrepCommand {
            pattern: String::from("foobar"),
            filename: PathBuf::from(r"./tests/test.file"),
            case_insensitive: true,
        };
        let cmd_fassade = commands::CommandsFassade::GrepCommand(grep_command);
        cmd_fassade.print();
        let _result = cmd_fassade.run().unwrap();
        assert_eq!(0, _result);
    }

    #[test]
    fn test_commandsfassade_grep_file_not_found() {
        let grep_command = grep::GrepCommand {
            pattern: String::from("Hello, world!"),
            filename: PathBuf::from(r"/file/does/not/exist/foobar"),
            case_insensitive: true,
        };
        let cmd_fassade = commands::CommandsFassade::GrepCommand(grep_command);

        cmd_fassade.print();
        let _result = cmd_fassade.run().unwrap_err();
        let err = error::ErrorType::Regular(error::ErrorKind::FileNotFound);
        assert!(matches!(err, _result));
    }
}
