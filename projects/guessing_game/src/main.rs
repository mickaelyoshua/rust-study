use std::io; // import form the standard library
use rand::Rng; // Crate 'rand' for randomization

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // 'thread_rng' returns the random number generator that is local to the current thread and
    // seeded by the OS.
    // the range syntax 'start..=end' is inclusive on the lower and upper bounds.

    println!("The secret number is: {secret_number}");

    println!("Please input your guess:");

    let mut guess = String::new();
    // 'let' statement creates a variable.
    // In Rust variable are immutable by default,
    // 'mut' statemant makes it mutable.
    //
    // 'String' is a growable string type from the std library.
    // The '::new' indicates that 'new' is an associated function of 'String'.
    // Associated function is a function that is implemented on a type.
    // The 'new' function creates a new empty string.

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // The 'read_line' function takes the input line from the user and append to a string.
    // '&' is for pass by reference. By default references are also immutable, so it is necessary to use '&mut' to make the reference mutable. 
    //
    // 'read_line' return a 'Result' type that is a enum that encodes error-handling information.
    // 'Result's variants are 'Ok' and 'Err'.
    //
    // If the 'Result' variant is 'Err', the 'expect' method will make the program crash and
    // display the message passed.
    // If the 'Result' variant is 'Ok', the 'expect' method will take the value that 'Ok' is
    // holding and return it. In this case is the number of bytes on the users input.

    println!("You guessed {guess}")
    // '{}' are placeholders for the formatted string.
    // E.g.: println!("x = {x} and y + 2 = {}", y + 2);
}
