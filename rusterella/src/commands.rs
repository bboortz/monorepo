use crate::grep;
use std::error::Error;
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

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        use crate::commands::CommandsFassade::GrepCommand;
        match self {
            GrepCommand(grep_struct) => {
                let config = grep::Config {
                    pattern: grep_struct.pattern.clone(),
                    filename: grep_struct.filename.clone(),
                    case_insensitive: grep_struct.case_insensitive,
                };
                grep::run(config)?;
            }
        }
        Ok(())
    }
}
