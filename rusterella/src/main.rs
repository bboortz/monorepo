use structopt::StructOpt;
mod commands;
mod grep;
mod logger;

static LOGGA: logger::Logga = logger::Logga {};

fn main() {
    println!("\n*** Program Start ***\n");
    let cmd = commands::CommandsFassade::from_args();
    if let Err(e) = cmd.run() {
        cmd.print();
        LOGGA.panic(&e);
    }
}
