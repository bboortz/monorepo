use structopt::StructOpt;
mod commands;
mod egrep;
pub mod error;
mod grep;
mod lib;
mod logger;

static LOGGA: logger::Logga = logger::Logga {};

/*
fn unbox<T>(value: Box<T>) -> T {
    *value
}
*/

fn main() -> Result<(), error::Error> {
    let cmd = commands::CommandsFassade::from_args();
    match cmd.run() {
        Ok(()) => Ok(()),
        Err(e) => {
            LOGGA.panic(&e);
            Err(e)
        }
    }
}
