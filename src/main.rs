//use environment library
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // get arguments and store
    let config = parse_config(&args);
    println!("query:{} and filename: {}",config.query,config.filename);
}
struct Config {
    query: String,
    filename: String,
}
fn parse_config(args: &[String]) -> Config{
    let query= args[1].clone();
    let filename= args[2].clone();
    Config {
        query,
        filename,
    }
}
