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

    let opt = opt::Opt::from_args();
    debug!("arguments: {:#?}", opt);

    if !opt.filename.exists() {
        let error_affected = opt.filename.to_str().unwrap_or("unknown file").to_string();
        let err = error::Error {
            error_type: error::ErrorType::Regular(error::ErrorKind::FileNotFound),
            affected: error_affected,
            suggestion: String::from(
                "Verify if the file exists and you have specified the filename correct.",
            ),
        };
        return Err(err);
    }

    let elf = file::parse_file(opt.filename)?;
    debug!("{:?}", elf);
    println!("{}", elf);
    /*
    match extract::extract_from_data(&input) {
        Some(data) => println!("data: {:?}", data),
        None => println!("no"),
    }
    */

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
