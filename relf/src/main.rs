mod display;
mod elf;
pub mod error;
mod file;
mod opt;
mod setup;
#[macro_use]
extern crate log;

// fn run() -> Result<(), Box<dyn Error>> {
fn run() -> Result<(), error::Error> {
    setup::setup();

    // read command line arguments
    let opt = opt::Opt::from_args();
    debug!("arguments: {:#?}", opt);

    // parse the provided file
    let elf = file::parse_file(&opt.filename)?;
    debug!("{:?}", elf);

    // print the result
    let filename = opt.filename.to_str().unwrap().to_string();
    println!("{:<10}: {:>12}", "FILENAME", filename);
    println!("{}", elf);

    Ok(())
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
