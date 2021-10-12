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

fn main() -> Result<(), error::ErrorType> {
    let cmd = commands::CommandsFassade::from_args();
    match cmd.run() {
        Ok(()) => return Ok(()),
        Err(e) => {
            // cmd.print();
            LOGGA.panic(&e);
            return Err(e);
        }
    };

    // Ok(())

    /*
    if let Err(e) = cmd.run() {
        cmd.print();
        LOGGA.print_type_of(&e);
        // LOGGA.panic(e);
    }
    */

    //let b = Box::new(5);
    //LOGGA.print_type_of(&b);
    //let v = unbox(b);
    //LOGGA.print_type_of(&v);
}
