//import necessary libraries
use std::env;
use std::process;
use minigrep::Config;


fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // display error msg and exit if error occurs
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //run main logic and handle error
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
