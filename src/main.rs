use duct::cmd;
use std::env;
use std::fs;
use std::path::Path;
use text_colorizer::*;

mod data;
mod printer;

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

    match parse_args() {
        Some(args) => match args.option.as_str() {
            // Add new arguments here
            "add" => add_file_to_trash(&args.filename, &trash_location, &trash_info_location),
            "restore" => restore_file(&args.filename, &trash_location, &trash_info_location),
            "clear" => clear_trash(&args.filename, &trash_location, &trash_info_location),
            "show" => show_file_list(&trash_info_location),
            _ => {
                println!(
                    "{} Unknown argument '{}'",
                    "Error:".red().bold(),
                    args.option.as_str()
                );
                printer::print_usage()
            }
        },
        None => {}
    }
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
    if filename == "all" {
        empty_trash(trash_location, trash_info_location);
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
    }

    match fs::File::create(trash_info_location) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("{} couldn't reset trash info file.", "Error:".red().bold());
            std::process::exit(1);
        }
    }
}

#[derive(Debug)]
struct Arguments {
    option: String,
    filename: String,
}

fn parse_args() -> Option<Arguments> {
    //TODO: Needs cleaning
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 1 && args[0] == "help" {
        // TODO: add help which prints all options
        println!("Help");
        return None;
    } else if args.len() == 1 && args[0] == "show" {
        return Some(Arguments {
            option: args[0].clone(),
            filename: "".to_string(),
        });
    } else if args.len() != 2 {
        printer::print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 2, got {}.",
            "Error:".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }

    Some(Arguments {
        option: args[0].clone(),
        filename: args[1].clone(),
    })
}
