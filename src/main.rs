//import necessary libraries
use std::env;
use std::process;
use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect(); // get arguments and store
    let config = Config::new(&args).unwrap_or_else(|err| {
        // display error msg and exit if error occurs
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    //display config
    println!("Searching for query: {}", config.query);
    println!("In file: {}", config.filename);

    //run main logic and handle error
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
