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

    // parse command line args
    /*
    let matches = build_args().get_matches();

    if let Some(matches) = matches.subcommand_matches("grep") {
        if matches.is_present("insensitive") {
            println!("case insensitive");
        } else {
            println!("case sensitive");
        }
    }
    */

    /*
    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", myfile);

    let num_str = matches.value_of("num");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => match s.parse::<i32>() {
            Ok(n) => println!("Your favorite number must be {}.", n + 5),
            Err(_) => println!("That's not a number! {}", s),
        },
    }
    */

    //
    // let args: Vec<String> = env::args().collect();

    // let config = grep::Config::new(cmd.pattern, cmd.filename, cmd.insensitive);
    /*
            &args).unwrap_or_else(|err| {
            eprintln!(
                "Problem during parsing commandline arguments: {}

    usage: {} <PATTERN> <FILE>
    ",
                err, args[0]
            );
            process::exit(1);
        });
    */
    /*
    println!("{:?}", config);

    if let Err(e) = grep::run(config) {
        LOGGA.panic(&e);
    }
    */
}
