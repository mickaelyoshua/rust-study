use std::env; // to read command line arguments
use std::fs; // to read the file

fn main() {
    // the first value in the vector is the name of the running binary
    let args: Vec<String> = env::args().collect(); // get all command line arguments
                                // args() return an iterator
                                // collect() get all values from iterator as a vector
                                // args have type explicit so the return from collect() is inferred
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for '{query}'");
    println!("In file '{file_path}'");

    let content = fs::read_to_string(file_path)
        .expect("Error reading file to string");

    println!("With content:\n\n{content}");
}
