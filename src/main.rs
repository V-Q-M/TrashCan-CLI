use duct::cmd;
use std::env;
use std::fs;
use std::io::Write;
use text_colorizer::*;

mod printer;

fn main() {
    let trash_location: String = "/home/vito/.trash".to_string();
    let trash_info_location: String = "/home/vito/.trashinfo".to_string();

    let args = parse_args();

    delete_file(&args.filename, &trash_location, &trash_info_location)
}

/// Restores a file from the trash can directory
fn restore_file(filename: &str) {}

/// Moves a file into the trashcan directory
fn delete_file(filename: &str, trash_location: &str, trash_info_location: &str) {
    save_fileinfo(&filename, &trash_info_location);

    match cmd!("mv", &filename, &trash_location).run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{} couldn't delete '{}'.", "Error:".red().bold(), &filename);
            std::process::exit(1);
        }
    }
}

use std::io;
use std::path::Path;
fn get_file_location(filename: &str) -> String {
    let path = Path::new(&filename);

    match path.canonicalize() {
        Ok(abs_path) => abs_path.display().to_string(),
        Err(e) => {
            eprintln!("{} finding file '{}': {e}", "Error:".red().bold(), filename);
            std::process::exit(1);
        }
    }
}

fn save_fileinfo(filename: &str, trash_info_location: &str) {
    let file_path: String = get_file_location(&filename);
    let data: String = format!("{}   {}", filename, file_path);

    match fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&trash_info_location)
    {
        Ok(mut file) => match writeln!(file, "{}", data) {
            Ok(_) => println!("Successfully appended."),
            Err(e) => println!("Write error: {e}"),
        },
        Err(e) => {
            eprintln!("Open error: {e}");
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
