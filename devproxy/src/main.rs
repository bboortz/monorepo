#[macro_use]
extern crate log;

use structopt::StructOpt;

mod client;
mod commands;
mod proxy;
mod server;
mod setup;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup::setup();

    let cmd = commands::CommandsFassade::from_args();
    match cmd.run() {
        Ok(()) => Ok(()),
        Err(e) => {
            error!("{}", &e);
            Err(e)
        }
    }
}
