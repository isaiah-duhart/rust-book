use std::{env, fs, io};

pub fn run(config: Config) -> Result<(), io::Error> {
    println!("Looking for {} in {}", config.query, config.file_path);

    let file_contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_sensitive(&config.query, &file_contents)
    } else {
        search_case_insensitive(&config.query, &file_contents)
    };

    for line in results{
        println!("{line}");
    }

    Ok(())
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    let query = query.to_lowercase();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Requires two args, query and file_path");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn test_search_fail() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
