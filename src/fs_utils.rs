use std::fs;
use std::path::Path;
use text_colorizer::*;

pub fn delete_file(file: &Path) {
    match fs::remove_file(file) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{} couldn't delete file: {}.", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    }
}

pub fn move_file(from: &Path, to: &Path) {
    match fs::copy(from, to) {
        Ok(_) => delete_file(&from),
        Err(e) => {
            eprintln!(
                "{} couldn't move file to new location: {}.",
                "Error:".red().bold(),
                e
            );
            std::process::exit(1);
        }
    }
}
