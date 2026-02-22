use duct::cmd;
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use text_colorizer::*;

mod data;
mod printer;

enum Command {
    Add,
    Restore,
    Clear,
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
            "clear" => Ok(Command::Clear),
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

/// Restores a file from the trash can directory
fn restore_file(filename: &str, trash_location: &str, trash_info_location: &str) {
    // read file old location
    let current_location = format!("{}/{}", trash_location, filename);
    let original_location = data::get_restore_location(filename, trash_info_location);

    match cmd!("mv", &current_location, &original_location).run() {
        Ok(_) => {
            data::remove_line_from_data(filename, trash_info_location);
        }
        Err(_) => {
            eprintln!(
                "{} couldn't restore '{}' from trash.",
                "Error:".red().bold(),
                &filename
            );
            std::process::exit(1);
        }
    }
}

/// Moves a file into the trashcan directory
fn add_file_to_trash(filename: &str, trash_location: &str, trash_info_location: &str) {
    data::save_file_data(&filename, &trash_info_location);

    match cmd!("mv", &filename, &trash_location).run() {
        Ok(_) => {}
        Err(_) => {
            eprintln!("{} couldn't delete '{}'.", "Error:".red().bold(), &filename);
            std::process::exit(1);
        }
    }
}

/// List files stored in trash directory by reading
fn show_file_list(trash_info_location: &str) {
    //TODO: Can be made prettier
    println!("NAME                  LOCATION");
    match cmd!("cat", &trash_info_location).run() {
        Ok(_) => {}
        Err(_) => {
            eprintln!(
                "{} couldn't get information about trashed files.",
                "Error:".red().bold()
            );
            std::process::exit(1);
        }
    }
}

fn clear_trash(filename: &str, trash_location: &str, trash_info_location: &str) {
    let trash_file_location = format!("{}/{}", trash_location, filename);
    let trash_file_path = Path::new(&trash_file_location);

    if trash_file_path.exists() {
        match fs::remove_file(trash_file_path) {
            Ok(_) => data::remove_line_from_data(filename, trash_info_location),
            Err(_) => {
                eprintln!(
                    "{} couldn't delete '{}' from trash.",
                    "Error:".red().bold(),
                    filename
                );
                std::process::exit(1);
            }
        }
    } else {
        eprintln!(
            "{} couldn't find '{}' in the trash.",
            "Error:".red().bold(),
            filename
        );
        std::process::exit(1);
    }
}

fn empty_trash(trash_location: &str, trash_info_location: &str) {
    let trash_path = Path::new(trash_location);

    if trash_path.exists() {
        match fs::remove_dir_all(trash_path) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("{} couldn't delete trashed files.", "Error:".red().bold());
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("{} Trash directory doesn't exist!", "Error:".red().bold());
        std::process::exit(1);
    }

    match fs::File::create(trash_info_location) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("{} couldn't reset trash info file.", "Error:".red().bold());
            std::process::exit(1);
        }
    }
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
        Command::Show => show_file_list(&trash_info_location),
        Command::Empty => empty_trash(&trash_location, &trash_info_location),
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
            add_file_to_trash,
            input,
            trash_location,
            trash_info_location,
        ),
        Command::Restore => eval_function(restore_file, input, trash_location, trash_info_location),
        Command::Clear => eval_function(clear_trash, input, trash_location, trash_info_location),
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
