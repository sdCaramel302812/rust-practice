use std::env;
use std::fs;
use std::process;
use std::error::Error;

pub fn grep_main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    // same as unwrap_or_else
    if let Err(e) = run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if (args.len() != 3) {
            return Err("Usaage:\n\t./practice quey filename");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone()
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // If error, ? operator will return error value
    let contents = fs::read_to_string(&config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    return Ok(());
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "prod";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
