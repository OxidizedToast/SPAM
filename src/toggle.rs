// SPDX-License-Identifier: Apache-2.0

use crate::config_dependancy_status;
use crate::config_file;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn toggle() {
    let home = std::env::var("HOME").unwrap();
    let config_dir_path = PathBuf::from(&home).join(".config").join("spam");
    let config_file_path = config_dir_path.join("config.toml");

    config_dependancy_status::check_if_empty(&config_file_path);
    // due to returns a bool it sets it as the value
    let daemon_actived = config_file::check_file_contents();
    // Confirms the directory exists
    fs::create_dir_all(&config_dir_path).unwrap();

    // toggles the config files boolean value
    if daemon_actived == true {
        println!("Toggling S.P.A.M Off");

        let mut file = fs::File::create(&config_file_path).unwrap();
        file.write_all(b"toggled = false").unwrap();
    } else if daemon_actived == false {
        println!("Toggling S.P.A.M On");

        let mut file = fs::File::create(&config_file_path).unwrap();
        file.write_all(b"toggled = true").unwrap();
    }
}
