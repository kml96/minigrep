//import necessary libraries
use std::{env, fs};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect(); // get arguments and store
    let config = Config::new(&args).unwrap_or_else(|err| {
        // display error msg and exit if error occurs
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    //display config
    println!("Searching for query: {}", config.query);
    println!("In file: {}", config.file);
}

// run code to display contents
fn run(config: Config) {
    //read contents from file
    let contents = fs::read_to_string(config.filename)
        .expect("Error reading file!"); // display error

    println!("With text:\n {}", contents); //display contents of file
}

// config
struct Config {
    query: String,
    filename: String,
}

// config implementation
impl Config {
    // return new instance of Config
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
