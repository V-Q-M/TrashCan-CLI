use std::env;
use text_colorizer::*;
use duct::cmd;

mod printer;


fn main() {
    let trash_location: String = "/home/vito/.trash".to_string();

    let args = parse_args();

    delete_file(args.filename, trash_location)
    
}

fn delete_file(filename: String, trash_location: String) {
    match cmd!("mv", &filename, &trash_location).run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error moving file to trashcan: {e}");
            std::process::exit(1);
        }
    }
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
            "{} wrong number of arguments: expected 2, got {}.",
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
