use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use text_colorizer::*;
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

/// Reads the trashinfo and returns the original location of the file
pub fn get_restore_location(filename: &str, trash_info_location: &str) -> String {
    match extract_restore_location_from_data(filename, trash_info_location) {
        Some(path) => path,
        None => {
            eprintln!(
                "{} '{}' not found in '{}'",
                "Error:".red().bold(),
                filename,
                trash_info_location
            );
            std::process::exit(1);
        }
    }
}

fn extract_restore_location_from_data(filename: &str, trash_info_location: &str) -> Option<String> {
    // read trash trash_info
    let data = match fs::read_to_string(trash_info_location) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to read from file '{}': {:?}",
                "Error:".red().bold(),
                trash_info_location,
                e
            );
            std::process::exit(1);
        }
    };

    data.lines().find_map(|line| {
        let mut parts = line.split_whitespace();
        let name = parts.next()?;
        let path = parts.next()?;

        if name == filename {
            Some(path.to_string())
        } else {
            None
        }
    })
}

pub fn remove_line_from_data(filename: &str, trash_info_location: &str) {
    // read trash trash_info
    let data = match fs::read_to_string(trash_info_location) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to read from file '{}': {:?}",
                "Error:".red().bold(),
                trash_info_location,
                e
            );
            std::process::exit(1);
        }
    };

    let mut found = false;

    let new_content: String = data
        .lines()
        .filter(|line| {
            let mut parts = line.split_whitespace();
            let name = parts.next();

            if name == Some(filename) {
                found = true;
                false
            } else {
                true
            }
        })
        .map(|line| format!("{line}\n"))
        .collect();

    if !found {
        eprintln!(
            "{} '{}' not found in '{}'",
            "Error:".red().bold(),
            filename,
            trash_info_location
        );
        std::process::exit(1);
    }

    match fs::write(trash_info_location, new_content) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to write to file '{}': {:?}",
                "Error".red().bold(),
                trash_info_location,
                e
            );
            std::process::exit(1);
        }
    }
}

pub fn save_file_data(filename: &str, trash_info_location: &str) {
    let file_path: String = get_file_location(&filename);
    let data: String = format!("{}  {}", filename, file_path);

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
