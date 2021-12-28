use std::env;
use std::fs;

fn main() {
    // match 1
    // handle Options https://doc.rust-lang.org/std/option/enum.Option.html using match
    match env::args().nth(1) {
        Some(v) => {
            println!("arg 1: {:?}", v);

            // match 2
            // handle Result https://doc.rust-lang.org/std/result/ using match
            match fs::read_to_string(&v) {
                Ok(contents) => println!("{}", contents),
                Err(e) => println!("{}", e),
            };
        }
        None => {
            println!("you have not passed an argument.");
        }
    }
}
