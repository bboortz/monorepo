use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
mod search;

#[derive(Debug, StructOpt)]
#[structopt(name = "grep", version = "0.1.0", about = "grep clone")]
pub struct GrepCommand {
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

impl GrepCommand {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(&self.filename)?;

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
    #[test]
    fn test_run_grep_command() {
        use crate::grep;
        // use crate::logger;
        use std::path::PathBuf;
        // let logga = logger::Logga {};
        let grep_command = grep::GrepCommand {
            pattern: String::from("Hello, world!"),
            filename: PathBuf::from(r"./test.file"),
            case_insensitive: false,
        };
        let result = grep_command.run().unwrap();
        assert_eq!((), result);
    }

    /*
    #[test]
    fn test_run_grep_command_file_not_found() {
        use crate::grep;
        use assert_str::assert_str_eq;
        use std::io;
        // use crate::logger;
        // use std::fs;
        use std::path::PathBuf;
        // let logga = logger::Logga {};
        let grep_command = grep::GrepCommand {
            pattern: String::from("Hello, world!"),
            filename: PathBuf::from(r"./unknown.file"),
            case_insensitive: false,
        };

        /*
        let result = grep_command.run().unwrap_err();
        assert_str_eq!("entity_not_found", result.description());
        */
        if let Err(e) = grep_command.run() {
            let expected = Err(io::ErrorKind::InvalidData);
            // let val: dyn std::error::Error = *e;
            // let val = (*e).clone();
            assert_eq!(expected, (&e).kind());
        }
        /*
                let result = grep_command.run().unwrap_err();
              let expected = Err(io::ErrorKind::InvalidData);
                assert_eq!(expected, result.kind());
        */
        /*
        let result = grep_command.run().unwrap_or_else(|err| {
            eprintln!("{}", err);
            assert_eq!(Err(e), err.kind());
        });
        */
        /*
        let Err(e) = fs::read_to_string(&"./unknown.file");
        */
        //        assert_eq!(Err(e), result.backtrace());
        //       assert_eq!(Err(e), result);
    }
    */

    /*
    if let Err(e) = grep::run(config) {
        println!("{}", e);
    }


    use crate::commands::CommandsFassade::GrepCommand;

    let cmd = GrepCommand { pattern: "foobar" };
    let arg_vec = vec!["my_prog", "grep", "pattern", "file"];
    let matches = App::new("myprog");
    eprintln!("{}", matches);

    let cmd = commands::CommandsFassade::from_args();
    if let Err(e) = cmd.run() {
        cmd.print();
    }
    */

    /*
    let mut out = Vec::new();
    let mut logger = Logger::new(&mut out);
    logger.log("Some warning");
    logger.flush();
    */
    /*
            eprintln!("{}", matches);
            matches.get_matches_from(arg_vec);
    */
    // Args and options go here...
    // let cmd = CommandsFassade::from_args();
    //        assert_eq!(vec!["safe, fast, productive."], search(pattern, contents));
}
