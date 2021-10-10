use crate::grep;
use std::error::Error;
use std::fmt::Debug;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "grep", version = "0.1.0", about = "grep clone")]
pub struct GrepStruct {
    /// activate case case_insensitive grep
    #[structopt(short = "i", long = "case_insensitive")]
    case_insensitive: bool,

    /// Specifies the pattern to search for
    #[structopt(name = "PATTERN")]
    pattern: String,

    /// Specifies the input file to use
    #[structopt(name = "FILE", parse(from_os_str))]
    filename: PathBuf,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rusterella",
    version = "0.1.0",
    about = "A single binary written in rust than combines several unix utilities. A busybox clone in rust.",
    author = "Benjamin Boortz <benjamn dot boortz at mailbox dot org>"
)]
pub enum Opts {
    #[structopt(name = "grep", version = "0.1.0", about = "grep clone")]
    GrepCommand(GrepStruct),
    /*
    Grep {
        /// activate case case_insensitive grep
        #[structopt(short = "i", long = "case_insensitive")]
        case_insensitive: bool,

        /// Specifies the pattern to search for
        #[structopt(name = "PATTERN", required_if("out_type", "file"))]
        pattern: String,

        /// Specifies the input file to use
        #[structopt(name = "FILE", parse(from_os_str))]
        filename: PathBuf,
    },
    */
}

impl Opts {
    pub fn print(&self) {
        use crate::opts::Opts::GrepCommand;
        match self {
            GrepCommand(grep_struct) => println!("GrepCommand: {:?}", grep_struct),
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        use crate::opts::Opts::GrepCommand;
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
