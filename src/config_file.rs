// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;
use std::fs;
use std::process;

pub fn check_file_contents() -> bool {

    let home = std::env::var("HOME").unwrap();
    let config_dir_path = PathBuf::from(&home).join(".config").join("spam");
    let config_file_path = config_dir_path.join("config.toml");

    let file_contents = fs::read_to_string(config_file_path)
        .unwrap_or_else(|err|{
            eprintln!("Failed to read config file: {}", err);
            process::exit(1);
        });

    if file_contents.contains("toggled = true"){
        true
    } else if file_contents.contains("toggled = false") {
        false
    } else {
        eprintln!("config file contents missing");
        process::exit(1);
    }
}
