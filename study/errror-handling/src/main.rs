use std::io::{self, Read};
use std::fs::File;
use std::error::Error;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
    // lines return an iterator for the lines on the text
    // next calls the next line, in this case the first one
    // chars return an iterator for every character on the line
    // last return the last char
}

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}
