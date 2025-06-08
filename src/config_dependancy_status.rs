// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::io::Write;
use std::path::PathBuf;

// checks for the directories and files
fn check_dir() {
    let home = std::env::var("HOME").unwrap();
    let config_dir_path = PathBuf::from(&home).join(".config").join("spam");

    if config_dir_path.exists() {
        check_file(&config_dir_path);
    } else {
        // creates directory (including parents)
        fs::create_dir_all(&config_dir_path).unwrap();
        check_file(&config_dir_path);
        println!("Config path created at: ~/.config/spam");
        println!("Please re-run to use!");
        std::process::exit(0);
    }
}

// function to check if the config file is made
fn check_file(config_dir_path: &PathBuf) {
    let config_file_path = config_dir_path.join("config.toml");

    if config_file_path.exists() {
        // file exists, does nothing
        check_if_empty(&config_file_path);
    } else {
        let mut file = fs::File::create(&config_file_path).unwrap();
        file.write_all(b"toggled = false").unwrap();

        println!("Config file created at ~/.config/spam/config.toml");
    }
}
pub fn check_if_empty(path: &PathBuf) {
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.len() == 0 {
                println!("Config file is empty.");
                println!("Writing default config");
                let mut file = fs::File::create(path).unwrap();
                file.write_all(b"toggled = false").unwrap();
            }
        }
        Err(e) => {
            eprintln!("Failed to get file metadata: {}", e);
            std::process::exit(1);
        }
    }
}

// cleaner name but keeps check dir name for readability when editing
pub fn status_file_check() {
    check_dir();
}
