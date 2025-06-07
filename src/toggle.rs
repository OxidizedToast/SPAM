// SPDX-License-Identifier: Apache-2.0
use crate::config_file;
use crate::config_dependancy_status;
use std::path::PathBuf;

pub fn toggle(){

    let home = std::env::var("HOME").unwrap();
    let config_dir_path = PathBuf::from(&home).join(".config").join("spam");
    let config_file_path = config_dir_path.join("config.toml");

    config_dependancy_status::check_if_empty(&config_file_path);
   // due to returns a bool it sets it as the value
    let daemon_actived = config_file::check_file_contents();
    if daemon_actived == true{
        // make file read to false
        println!("Toggling S.P.A.M Off");
    } else if daemon_actived == false {
       // make file read to true 
        println!("Toggling S.P.A.M On");
    }
}
