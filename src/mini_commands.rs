// SPDX-License-Identifier: Apache-2.0

pub fn help(){
    println!(
        r#"
 ██████╗       ███████╗██████╗  █████╗ ███╗   ███╗
██╔═══██╗      ██╔════╝██╔══██╗██╔══██╗████╗ ████║
██║   ██║█████╗███████╗██████╔╝███████║██╔████╔██║
██║   ██║╚════╝╚════██║██╔═══╝ ██╔══██║██║╚██╔╝██║
╚██████╔╝      ███████║██║     ██║  ██║██║ ╚═╝ ██║
 ╚═════╝       ╚══════╝╚═╝     ╚═╝  ╚═╝╚═╝     ╚═╝

    Commands:
        -h  =>  Prints this help screen
        -v  =>  Prints version of project
        -t  =>  Toggles if daemon is on
 "#
    );
}

pub fn version(){
    let version = env!("CARGO_PKG_VERSION");
    println!("Project version: {}", version);
}
