use std::env; use std::error::Error;
// to read command line arguments
use std::fs;
use std::process; // to read the file

use minigrep::search;

fn main() {
    // the first value in the vector is the name of the running binary
    let args: Vec<String> = env::args().collect(); // get all command line arguments
                                // args() return an iterator
                                // collect() get all values from iterator as a vector
                                // args have type explicit so the return from collect() is inferred

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    }); // non-panic handling error

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    } // execute if run return an error value
    // made this way because it only matter if returns an error or not
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Box<dyn Error> means that will return a type the implements the Error trait
    
    let content = fs::read_to_string(config.file_path)?;
    // ? will return the error value from the current function
    
    for line in search(&config.query, &content) {
        println!("{line}");
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        }
    }

    // 'static lifetime is a special lifetime that defines that the reference will live for the
    // entire duration of the program
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        
        Ok(Config::new(args))
    }
}
