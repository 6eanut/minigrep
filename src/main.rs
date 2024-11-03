use minigrep;
use std::env;
use std::process;
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
