//import necessary libraries
use std::fs;
use std::error::Error;
use std::env;

// config
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// config implementation
impl Config {
    // return new instance of Config
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Error getting query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Error getting filename")
        };

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// run code to display contents
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //read contents from file
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        case_insensitive_search(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

// return lines containing given query(case sensitive search)
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// return lines containing given query(case insensitive search)
pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}


//tests
#[cfg(test)]
mod tests {
    use super::*;

    //case sensitive test
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    //case insensitive test
    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], case_insensitive_search(query, contents));
    }
}
