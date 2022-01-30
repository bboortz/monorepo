#[macro_use]
extern crate log;

use std::thread;
use std::time::Duration;

use fltk::{app, enums::FrameType, prelude::*, *};

mod conntest;
mod error;
mod reqwest_client;
mod setup;
#[cfg(target_os = "windows")]
mod systray;

const TIMEOUT: std::time::Duration = Duration::from_millis(10000);

fn run() -> Result<(), error::Error> {
    setup::setup();

    loop {
        conntest::run()?;
        thread::sleep(TIMEOUT);
    }

    // Ok(())
}

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        }
    }
}
