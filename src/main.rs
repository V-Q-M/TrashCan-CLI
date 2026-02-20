use duct::cmd;
use std::env;
use std::fs;
use std::io::Write;
use text_colorizer::*;

mod data;
mod printer;

fn main() {
    let trash_location: String = "/home/vito/.trash".to_string();
    let trash_info_location: String = "/home/vito/.trashinfo".to_string();

    match parse_args() {
        Some(args) => match args.option.as_str() {
            // Add new arguments here
            "add" => delete_file(&args.filename, &trash_location, &trash_info_location),
            "restore" => restore_file(&args.filename, &trash_location, &trash_info_location),
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
        Err(e) => {
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
fn delete_file(filename: &str, trash_location: &str, trash_info_location: &str) {
    data::save_file_data(&filename, &trash_info_location);

    match cmd!("mv", &filename, &trash_location).run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{} couldn't delete '{}'.", "Error:".red().bold(), &filename);
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
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 1 && args[0] == "help" {
        // TODO: add help which prints all options
        println!("Help");
        return None;
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
