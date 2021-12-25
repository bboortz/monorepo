use structopt::StructOpt;
mod commands;
mod egrep;
pub mod error;
mod grep;
mod lib;
mod logger;
mod secfind;
mod secgrep;

static LOGGA: logger::Logga = logger::Logga {};

/*
use std::process;
fn unbox<T>(value: Box<T>) -> T {
    *value
}

impl process::Termination for process::ExitCode {
    #[inline]
    fn report(self) -> i32 {
            self.0.as_i32()
        }
}
*/

fn main() -> Result<(), error::Error> {
    let cmd = commands::CommandsFassade::from_args();
    /*
    match cmd.run() {
        Ok(n) => Ok(()),
        Err(e) => {
            LOGGA.panic(&e);
            Err(e)
        }
    }
    */

    std::process::exit(match cmd.run() {
        Ok(n) => n,
        Err(e) => {
            LOGGA.panic(&e);
            1
        }
    });
}
