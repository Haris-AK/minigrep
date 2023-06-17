use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            panic!("You have not entered enough arguments")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // Checks for the IGNORE_CASE env variable, if it is set, the is_ok() method runs on a
        // Result, and will return false if the env variable is not set
        //
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line)
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    println!("{}", &query);
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        let query = "req";
        let dummy_data = "\
This is some text.
It has the word required in it
This reQUIred should be ignored.
            ";
        assert_eq!(
            vec!["It has the word required in it"],
            search(query, dummy_data)
        )
    }

    #[test]
    fn case_sensitive() {
        let query = "HAS";
        let dummy_data = "\
This is some text.
It has the word required in it
This reQUIred should be ignored.
            ";
        assert_ne!(
            vec!["It has the word required in it"],
            search(query, dummy_data)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "SOmE";
        let dummy_data = "\
This is some text.
It has the word required in it
            ";
        assert_eq!(
            vec!["This is some text."],
            search_case_insensitive(query, dummy_data)
        )
    }
}
