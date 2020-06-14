//use environment library
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // get arguments and store
    let query = &args[1]; // query string to search
    let filename = &args[2]; // filename to search

    println!("query string is {}",query);
    println!("filename to search is {}",filename);
}
