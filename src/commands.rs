// commands.rs

use std::fs;
use dirs;
use colored::*;
use crate::utils::zip_shbin;
use crate::utils::post_shbin;
use crate::utils::rm_zip;

// TODO: Make file operations async
pub fn ls() {
    let shbin_path = dirs::home_dir().unwrap().join(".shbin");
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

// TODO: Make file operations async
pub fn add(path: &str) {
    let shbin_path = dirs::home_dir().unwrap().join(".shbin");
    let new_file_path = shbin_path.join(path);

    let _ = fs::copy(path, new_file_path);

    println!("Added {:?}!", path);
    ls();
}

// TODO: Make file operations async
pub fn rm(path: &str) {
    let shbin_path = dirs::home_dir().unwrap().join(".shbin");
    let file_path = shbin_path.join(path);
    
    let _ = fs::remove_file(file_path);

    println!("Removed {:?}!", path);
    ls();
}

pub async fn push() {
    // compress shbin directory
    zip_shbin();

    // post compressed file
    post_shbin().await;

    // delete zip
    rm_zip().await;

}