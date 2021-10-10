use rusterella::Config;
use std::env;

use std::process;
mod logger;

static LOGGA: logger::Logga = logger::Logga {};

fn main() {
    println!("\n*** Program Start ***\n");

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!(
            "Problem during parsing commandline arguments: {}

usage: {} <PATTERN> <FILE>
",
            err, args[0]
        );
        process::exit(1);
    });
    println!("{:?}", config);

    if let Err(e) = rusterella::run(config) {
        LOGGA.panic(&e);
    }
}
