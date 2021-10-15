use structopt::StructOpt;
mod commands;
pub mod error;
mod grep;
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
