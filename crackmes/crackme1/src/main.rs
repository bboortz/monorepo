use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let flag = String::from("CTFlearn{y0uM4d31t}");

    if args.len() < 2 {
        eprintln!("usage: {} <ARG>", args[0]);
        process::exit(1);
    }

    if args[1] != "let me in !!!" {
        eprintln!("wrong argument: {}", args[1]);
        process::exit(1);
    }
    eprintln!("you got it!");
    eprintln!("The flag: {}", flag);
}
