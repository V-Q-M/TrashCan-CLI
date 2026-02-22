use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use text_colorizer::*;

mod data;
mod printer;
mod ops;

enum Command {
    Add,
    Restore,
    Delete,
    Empty,
    Help,
    Show,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(Command::Add),
            "restore" => Ok(Command::Restore),
            "delete" => Ok(Command::Delete),
            "empty" => Ok(Command::Empty),
            "help" => Ok(Command::Help),
            "show" => Ok(Command::Show),
            _ => Err(format!(
                "{} '{}' is not a valid command.",
                "Error:".red().bold(),
                s
            )),
        }
    }
}

fn main() {
    let home_dir = env::var("HOME").expect("Could not find HOME directory.");

    let trash_location = format!("{}/.trash", home_dir);
    let trash_info_location = format!("{}/.trashinfo", home_dir);

    if !Path::new(&trash_location).exists() {
        fs::create_dir(&trash_location).expect("Failed to create trash directory");
        println!(
            "Created {} directory in your HOME directory.",
            trash_location
        );
    } else {
        // println!("directory already exists: {}", trash_location);
    }

    if !Path::new(&trash_info_location).exists() {
        fs::File::create(&trash_info_location).expect("Failed to create trashinfo file");
        println!(
            "Created {} file in your HOME directory.",
            trash_info_location
        );
    } else {
        // println!("File already exists: {}", trash_info_location);
    }

    parse_args(&trash_location, &trash_info_location);
}



/*
#[derive(Debug)]
struct Arguments {
    option: String,
    filename: String,
}
*/

fn eval_single_argument(command: Command, trash_location: &str, trash_info_location: &str) {
    match command {
        Command::Help => println!("Help"), // TODO: add help which prints all options
        Command::Show => ops::show_file_list(&trash_info_location),
        Command::Empty => ops::empty_trash(&trash_location, &trash_info_location),
        _ => invalid_arguments(2 as usize, 1 as usize),
    }
}

fn eval_function(
    func: fn(&str, &str, &str),
    input: &Vec<String>,
    trash_location: &str,
    trash_info_location: &str,
) {
    let mut i: usize = 1;
    while i < input.len() {
        let filename: &str = &input[i].clone();
        func(&filename, &trash_location, &trash_info_location);
        i += 1;
    }
}

fn eval_multi_argument(
    command: Command,
    input: &Vec<String>,
    trash_location: &str,
    trash_info_location: &str,
) {
    match command {
        Command::Add => eval_function(
            ops::add_file_to_trash,
            input,
            trash_location,
            trash_info_location,
        ),
        Command::Restore => eval_function(ops::restore_file, input, trash_location, trash_info_location),
        Command::Delete => eval_function(ops::delete_from_trash, input, trash_location, trash_info_location),
        _ => invalid_arguments(1 as usize, input.len()),
    }
}

fn invalid_arguments(expected: usize, got: usize) {
    printer::print_usage();
    eprintln!(
        "{} wrong number of arguments: expected {}, got {}.",
        "Error:".red().bold(),
        expected,
        got
    );
    std::process::exit(1);
}

fn parse_args(trash_location: &str, trash_info_location: &str) {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        invalid_arguments(1 as usize, args.len());
        return;
    }
    let command_str = &args[0];
    let command = match command_str.parse::<Command>() {
        Ok(cmd) => cmd,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    match args.len() {
        1 => eval_single_argument(command, trash_location, trash_info_location),
        2.. => eval_multi_argument(command, &args, trash_location, trash_info_location),
        n => invalid_arguments(1 as usize, n),
    }
}
