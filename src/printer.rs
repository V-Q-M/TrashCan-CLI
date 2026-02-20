use text_colorizer::*;

pub fn print_usage() {
    eprintln!(
        "{} - Move files to trash or restore them from the trashcan",
        "trashcan".green()
    );
    eprintln!("Usage: trashcan <option> <filename>");
}
