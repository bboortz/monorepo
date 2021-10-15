use crate::error;
use crate::grep;
use std::fmt::Debug;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rusterella",
    version = "0.1.0",
    about = "A single binary written in rust than combines several unix utilities. A busybox clone in rust.",
    author = "Benjamin Boortz <benjamn dot boortz at mailbox dot org>"
)]
pub enum CommandsFassade {
    #[structopt(name = "grep", version = "0.1.0", about = "grep clone")]
    GrepCommand(grep::GrepCommand),
}

impl CommandsFassade {
    pub fn print(&self) {
        use crate::commands::CommandsFassade::GrepCommand;
        match self {
            GrepCommand(grep_struct) => println!("GrepCommand: {:?}", grep_struct),
        }
    }

    /*
    pub fn run(&self) -> Result<(), Box<error::ErrorType>> {
    */
    pub fn run(&self) -> Result<(), error::Error> {
        use crate::commands::CommandsFassade::GrepCommand;
        match self {
            GrepCommand(grep_command) => {
                grep_command.run()?;
            }
        }
        Ok(())
    }
}

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
        assert_eq!((), _result);
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
