use std::env;
use std::error::Error;
// to read command line arguments
use std::fs;
use std::process; // to read the file

use minigrep::search;
use minigrep::search_case_insensitive;
use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    }); // non-panic handling error

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    } // execute if run return an error value
    // made this way because it only matter if returns an error or not
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Box<dyn Error> means that will return a type the implements the Error trait
    
    let contents = fs::read_to_string(config.file_path)?;
    // ? will return the error value from the current function
    
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    for line in result {
        println!("{line}");
    }

    Ok(())
}

