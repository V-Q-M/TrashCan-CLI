use std::env;
use text_colorizer::*;

mod printer;

fn main() {
    let args = parse_args();
    println!("{:?}", args);
}

#[derive(Debug)]
struct Arguments {
    option: String,
    filename: String,
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        printer::print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 4, got {}.",
            "Error:".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }

    Arguments {
        option: args[0].clone(),
        filename: args[1].clone(),
    }
}
