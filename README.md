# minigrep

grep : Global Regular Expression Print

## Usage

[CASE_INSENSITIVE=1] cargo run query filename.txt

## Function

Print the lines in the file, which contain the query

## Implement

### data struct

```rust
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
```

* Defined in src/lib.rs
* members
  * query : the query the command line contains
  * filename ： the filename the command line contains
  * case_sensitive : true or false, which is determined by the env::var CASE_INSENSITIVE
* method
  * new
    * parameter type : Vec `<String>`, the command line, which is processed by src/main.rs/main
    * return type : Result <Config,String>, the command line must contain three arguments

### Search

```rust
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
```

### Run

```rust
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
```

### Main

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = minigrep::Config::new(args).unwrap_or_else(|err| {
        println!("Problem in parsing arguments:\n{}", err);
        process::exit(1);
    });
    println!("content = {}, file = {}", config.query, config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Problem in reading file:\n{}", e);
        process::exit(1);
    }
}
```

### Test

```rust
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
```
