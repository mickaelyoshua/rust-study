# Developer tools
* **Cargo:** Dependency manager and build tool
* **Rustfmt:** Formatting tool
* **rust-analyser:** LSP for Rust

# Install, Update, Uninstall and Documentation
* Install
Latest stable version of `rustup`, a command line tool to manage Rust:
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

```

* Update
```bash
rustup update
```

* Uninstall
```bash
rustup self uninstall
```

* Documentation
```bash
rustup doc

```
Will open the documentation on your browser

# Compiler
Rust is a *ahead-of-time* compiled language, so you can compile a program and give the executable to another person and they can run on another (compatible to the binary) system even without Rust is installed.

# Cargo
Rust's buid system and package manager. It builds the code, download and build dependencies.
* Create new project with cargo:
```bash
cargo new hello_cargo

```
It creates a new directory called "hello_cargo" and ohter file and a directory inside. It automaticaly crated a git project with a *.gitignore* unless is already inside a git repo.
The created files are:
* **Cargo.toml:** A configuration file for Cargo
* **src/main.rs:** Initial file with a "Hello, world!" print
An easy way to get a `Cargo.toml` file is runnig `cargo init` that generates a new configuration file.

Cargo has a mechanism that ensures you can rebuild the same artifact every that that youur code is builded: It will use only the specified versions of the dependencies until it is indicaded otherwise.
To handle this is created the `Cargo.lock` file the first time `cargo build` is executed so it register and uses always the same versions of the dependencies and the one that the the dependencies depend.

To know more about dependencies functionalities run `cargo doc --open` to open the documentation of your dependencies.

## Crates
Crates are packages of codes that can be included on your projects (external libraries).
[Rust external packages - Crates](https://crates.io)

To update a crate run `cargo update`, which will ignore the `Cargo.lock` and get all the latest versions that fit in the `Cargo.toml` file and then overwrite on `Cargo.lock`.

## Building and runnig with
By running `cargo build` it will create a executable file at `target/debug/hello_cargo` and many other files. The default build is a debug build.
To compile and execute the code just run `cargo run`. It is a `cargo build` followed by the execution of the binary.

To check if the code will compiles but without generating an executable tun `cargo check`, it runs faster then `cargo build`.

When project is ready for release, run `cargo build --release` to compile with optimizations. Will create an executable and other files ate `target/release`.

[Cargo Documentation](https://doc.rust-lang.org/cargo/)

# Std library
[Standard library documentation](https://doc.rust-lang.org/std/prelude/index.html)

# Diff between Default Immutable Variables and Constants
Constants (`const`) can be declared at any scope, it is not possible to declare a variable with `let` ate the global level.
```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("Three hours have {THREE_HOURS_IN_SECONDS} seconds")
}

```

# Shadowing
It is possible to *shadow* a varible. You declare a variable then creates a new variable with the same name *shadowing* the first one, it takes any uses of the variable name on the scope where it was redeclared.
```rust
fn main() {
    let x = 5;
    let x = x + 1; // shadowing

    {
        let x = x * 2; // shadowing again
        println!("The value of x in inner scope is: {x}"); // 12
    }

    println!("The value of x is: {x}"); // 6
}

```
It also can be done with different datatypes, it is reusing the name on the scope.

# Data Types
## Char
Chars on Rust are not ASCII, they are Unicode.

## Tuples
is a grouping of a variety of types. It can not change it size.
Creating a tuple:
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

It is possible to get the values of a tuple by *destructuring* it:
```rust
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}"); // 6.4
}
```

It is also possible to access tuple values by indexing with a '.':
```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

A tuple without values is called *unit*. This value and its corresponding type are both written `()` and represent an empty return type. Expression implicitly return the *unit* value if not returning any other value.

## Arrays
In Rust, arrays have a fixed length. The declaration of an arrays is:
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

An arrays type annotation, definition of type and length:
```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
```

Initialize an array with all elements equal:
```rust
fn main() {
    let a = [3; 5]; // length 5 and all elements are the number 3
}
```

Access elements of an arrays:
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0]; // 1
    let second = a[1]; // 2
}
```

# Functions
The conventional naming style for Rust is the *snake case* in with all letters are lowercase and the words are separeted with underscore.

## Statements and Expressions
Rust is a expression-based language. The definitions of statement and expression are:
* **Statements:** Instructions that perform some action and do not return a value
* **Expressions:** Evaluate to a result value
```rust
fn main() {
    let y = 6; // Statement - do not return values
    // In other languages is possible to do x = y = 6,
    // but in Rust it is not possible, the statement does not return
    // a value to be assigned to another variable.
}
```
On expressions it has a value to return. Calling a function is an expression, calling a macro also is, a new scope is also a expression.
```rust
fn main() {
    let y = {
        let x = 5;
        x+1 // Expressions do not include a semicolon at the end.
            // If a semicolon is added, the expression turns into a statement.
    }; // This scope is an expression.
    // The value to y will be 6
    // x+1 returns a value and the value
    // will be assigned to y.
}
```

## Returns
Returns of functions on Rust always return the un-semicoloned final expression on the block or with the `return` word.
```rust
fn plus_one(x: i32) -> i32 {
    x + 1
}
fn plus_one_with_return(x: i32) -> i32 {
    return x + 1
}

fn main() {
    let p1 = plus_one(5);
    let p2 = plus_one_with_return(6);
    println!("Plus one: {p1}");
    println!("Plus one with return: {p2}");
}
```

# Control Flow
## `if` statements
Unlike other languages, in Rust it will not convert non-boolean types to a boolean. On a if statement the condition must return a boolen or be a boolean variable.

## `if` in a `let` statement
Since `if` is an expression, it can return thing, so it is possible to use on a `let` statement.
```rust
fn main() {
    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("Value of number is: {number}"); // 6
}
```

## Repetition wit loops
### `loop`
`loop` is also a expression, witch means it can return a value to be assingned.
```rust

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // the value after the 'break' will be returned
        }
    };

    println!("The result is {result}");
}
```

### Loop Labels to Disambiguate Between Multiple Loops
It is possible to specify a loop for a `break` or `continue` command. The way to do it is with labels.
```rust
fn main() {
    let mut count = 0;

    'counting_up: loop { // naming the loop
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // break the inner loop
            }

            if count == 2 {
                break 'counting_up; // break external loop
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");
}
```

## Loop Through a Collection with `for`
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("Value is: {element}");
    }
}
```

Looping on a range:
```rust
fn main() {
    for number in (1..4).rev() {
        // a range from start to end-1, 'rev()' inverts the range
        println!("{number}!");
    }
    println!("LIFTOFF!!!!");
}
```


