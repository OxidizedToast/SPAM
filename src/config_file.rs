// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;

pub fn check_file_contents() -> bool {
    let home = std::env::var("HOME").unwrap();
    let config_dir_path = PathBuf::from(&home).join(".config").join("spam");
    let config_file_path = config_dir_path.join("config.toml");

    let file_contents = fs::read_to_string(&config_file_path).unwrap_or_else(|err| {
        eprintln!("Failed to read config file: {}", err);
        process::exit(1);
    });

    if file_contents.contains("toggled = true") {
        true
    } else if file_contents.contains("toggled = false") {
        false
    } else {
        println!("Config file contents missing");
        println!("Set default config? [Y/n]");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim().to_uppercase() == "Y" {
            println!("Adding default config!");

            // Ensure the config directory exists
            fs::create_dir_all(&config_dir_path).unwrap();

            // Now write default config
            let mut file = fs::File::create(&config_file_path).unwrap();
            file.write_all(b"toggled = false").unwrap();
        } else {
            println!("Exiting");
        }

        process::exit(1);
    }
}
