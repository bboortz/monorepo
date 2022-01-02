use std::fmt::Debug;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "relf",
    version = env!("CARGO_PKG_VERSION"),
    about = "ELF file analyzer written in rust.",
    author = "Benjamin Boortz <benjamn dot boortz at mailbox dot org>"
)]
pub struct Opt {
    /// activate debug mode
    #[structopt(short = "d", long = "debug")]
    pub debug: bool,

    /// activate verbose mode
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    /// Specifies the input file to use
    #[structopt(name = "FILE", parse(from_os_str))]
    pub filename: PathBuf,
}

impl Opt {
    pub fn from_args() -> Opt {
        <Opt as StructOpt>::from_args()
    }
}
