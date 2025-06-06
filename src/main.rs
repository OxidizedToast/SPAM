// SPDX-License-Identifier: Apache-2.0

use std::env;
mod mini_commands;
mod config_dependancy_status;
mod config_file;
mod toggle;

fn main() {
    // Checks if operating system supported
    const USERS_OPERATING_SYSTEM :&str = std::env::consts::OS;
    match USERS_OPERATING_SYSTEM {
        "linux" => (),
        _ => println!("Sorry operating system {} not supported\n exiting...", USERS_OPERATING_SYSTEM)
    }
    // Checks status on the config dir/files if they exist
    config_dependancy_status::status_file_check();

    // Gets user arguments
    let args: Vec<String> = env::args().collect();
    // checks if their is more than 1 argument given to terminal, due to first argument pregiven so
    // if its equal to one then no input was given
    if args.len() > 1 {
        match args[1].as_str() {
            "-h" => mini_commands::help(),
            "-v" => mini_commands::version(), 
            "-t" => toggle::toggle(),
            _ => println!("Command {} not recognized, try '-h' for help", args[1]),
        }
    } else {
        println!("No command provided, try '-h' for help")
    }
}
