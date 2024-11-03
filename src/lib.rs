use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let filecontent = fs::read_to_string(config.filename)?;
    let results;
    // println!("file : \n{}", names);
    if config.case_sensitive {
        results = search_sensitive(&config.query, &filecontent);
    } else {
        results = search_insensitive(&config.query, &filecontent);
    }
    println!("{:?}", results);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, String> {
        if args.len() == 3 {
            return Err(String::from("Usage : cargo run query file"));
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

fn search_sensitive<'a>(query: &'a str, filecontent: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in filecontent.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_insensitive<'a>(query: &'a str, filecontent: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query =query.to_lowercase();
    for line in filecontent.lines() {
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
    fn test00() {
        let query = "john";
        let filecontent = "tom has many friends.\njohn likes apples.\ntom is my brother.\nbill must go to school.\n";
        assert_eq!(vec!["bill must go to school"], search_sensitive(query, filecontent));
    }
}
