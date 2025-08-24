use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(query: String, file_path: String) -> Config {
        Config {
            query,
            file_path,
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        }
    }

    // 'static lifetime is a special lifetime that defines that the reference will live for the
    // entire duration of the program
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        args.next(); // skip the first one
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path."),
        };

        Ok(Config::new(query, file_path))
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase(); // this will allocate a new String

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
