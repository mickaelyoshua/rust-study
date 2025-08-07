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

