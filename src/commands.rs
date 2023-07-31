use std::fs;
use dirs;
use colored::*;

pub fn ls() {
    let home_dir = dirs::home_dir().unwrap();
    let shbin_path = home_dir.join(".shbin");

    let entries = fs::read_dir(shbin_path).unwrap();

    println!("{:<6} {:<20} {:<6}", 
             "Index".cyan(), 
             "Name".green(), 
             "Size".yellow());

    for (i, entry_result) in entries.enumerate() {
        let entry = entry_result.unwrap();
        let file_size = format!("{}B", entry.metadata().unwrap().len().to_string()).yellow();

        println!("{:<6} {:<20} {:<6}",
                 i.to_string().cyan(), 
                 entry.file_name().to_string_lossy().green(), 
                 file_size);
    }
}

pub fn add(path: &str) {
    println!("Adding {:?}", path);
    // TODO: implement the add functionality here
}

pub fn rm(path: &str) {
    println!("Removing {:?}", path);
    // TODO: implement the rm functionality here
}

pub fn push() {
    println!("You typed push");
    // TODO:implement the push functionality here
}