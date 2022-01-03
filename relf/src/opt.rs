use std::fmt::Debug;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opts() {
        assert_eq!(
            Opt {
                debug: false,
                verbose: false,
                filename: std::path::PathBuf::from(r"./samples/handcrafted")
            },
            Opt::from_clap(&Opt::clap().get_matches_from(&["relf", "./samples/handcrafted"]))
        );

        assert_eq!(
            Opt {
                debug: false,
                verbose: true,
                filename: std::path::PathBuf::from(r"./samples/handcrafted")
            },
            Opt::from_clap(&Opt::clap().get_matches_from(&["relf", "-v", "./samples/handcrafted"]))
        );

        assert_eq!(
            Opt {
                debug: true,
                verbose: false,
                filename: std::path::PathBuf::from(r"./samples/handcrafted")
            },
            Opt::from_clap(&Opt::clap().get_matches_from(&["relf", "-d", "./samples/handcrafted"]))
        );
    }
}
