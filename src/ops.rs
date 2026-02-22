use duct::cmd;
use std::fs;
use std::path::Path;
use text_colorizer::*;

use crate::data;

/// Restores a file from the trash can directory
pub fn restore_file(filename: &str, trash_location: &str, trash_info_location: &str) {
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
pub fn add_file_to_trash(filename: &str, trash_location: &str, trash_info_location: &str) {
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
pub fn show_file_list(trash_info_location: &str) {
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

pub fn delete_from_trash(filename: &str, trash_location: &str, trash_info_location: &str) {
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

pub fn empty_trash(trash_location: &str, trash_info_location: &str) {
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
