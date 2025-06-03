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
 "#
    );
}

pub fn version(){
    let version = env!("CARGO_PKG_VERSION");
    println!("Project version: {}", version);
}
