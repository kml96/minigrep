//use environment library
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // get arguments and store
    let config = Config::new(&args); // get instance of config
    println!("query:{} and filename: {}", config.query, config.filename); //display query and filename
}

// config
struct Config {
    query: String,
    filename: String,
}

// config implementation
impl Config {
    // return new instance of Config
    fn new(args: &[String]) -> Config {
        if args.len()<3 {
            panic!("Not enough arguments. Please input  both query and filename");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Config {
            query,
            filename,
        }
    }
}
