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

# Ownership
Some languages have garbage collectors to manage memory, looking for a no longer space used to free it as the program runs (like Python), others the memory must be explicitly allocated and freed.
In Rust there is a third way to do it: memory is managed through a system of ownership with a set of rules that the compile checks. If any of these rules are violated, the program won't compile. None of the features of ownership will slow down the program when it is running.

## Stack and Heap
The stack and the heap are two distinct regions of memory available to a program at runtime, each with different structures and use cases. The choice between them is crucial in systems programming as it affects performance and memory management.

---

### The Stack

The stack is a highly organized region of memory that operates on a **Last-In, First-Out (LIFO)** principle.

* **Structure and Operations**: Data is added to the top of the stack in a process called **pushing**, and removed from the top in a process called **popping**. This rigid order makes memory management simple and fast.
* **Data Type**: All data stored on the stack must have a **known, fixed size** determined at compile time. This includes integers, floating-point numbers, characters, booleans, and pointers.
* **Management**: Memory on the stack is managed automatically. When a function is called, a block of memory called a *stack frame* is pushed onto the stack to store its local variables and parameters. When the function finishes, this frame is popped off, automatically deallocating that memory.
* **Performance**: Pushing and popping are extremely fast operations because there's no need to search for a place to store data; it's always at the top. This also leads to excellent *locality of reference*, meaning data is stored contiguously, which allows modern CPUs to access it very quickly via their caches. A common error associated with the stack is a *stack overflow*, which occurs when it runs out of space.
---

### The Heap

The heap is a less organized pool of memory used for data whose size may be unknown at compile time or might change during the program's execution.

* **Structure and Operations**: When data needs to be stored on the heap, the program requests a certain amount of space from the system's *memory allocator*. The allocator finds a suitable empty block, marks it as in use, and returns a *pointer* (the memory address) to that location. This process is called *allocation*.
* **Data Type**: The heap is used for dynamic data, such as strings that can grow, vectors, or complex user-defined data structures.
* **Management**: Heap memory must be managed explicitly. In languages like C/C++, the programmer must manually deallocate (free) the memory once it's no longer needed. Forgetting to do so results in a *memory leak*. In other languages like Java, Python, or Rust (through its ownership system), this process is handled automatically by a garbage collector or other mechanisms.
* **Performance**: Allocating memory on the heap is slower than pushing to the stack because the allocator must perform work to find a sufficiently large block of free memory. Accessing data on the heap is also typically slower because it requires an extra step of *dereferencing* a pointer. Data on the heap can be fragmented, leading to poor locality of reference and slower access times.
---

### Key Differences Summarized

| Feature | Stack | Heap |
| :--- | :--- | :--- |
| **Speed** | Very fast allocation and access | Slower allocation and access |
| **Data Size** | Fixed, known at compile time | Dynamic, can change at runtime |
| **Management** | Automatic (LIFO) | Manual or via a garbage collector |
| **Structure** | Highly organized, contiguous | Less organized, fragmented |
| **Access** | Direct | Indirect (via pointers) |


Keeping track of what parts of code are using data on the heap, minimizing the ammount of duplicated data on the heap, cleaning unused data on the heap are all problems that ownership addresses.

## Ownership rules
* Each value in Rust has an *owner*;
* There can be only one owner at a time;
* When the owner goes out of scope, the value will be dropped.

For example, the string literals are allocated on the *stack* and can not be changed. But the `String` type is allocated on the *heap* and can be modifyed.
```rust
fn main() {
    let s = "hello"; // this is a string literal, is immutable and allocated on the stack
    println!("{s}");

    let s = String::from(s); // this is a String type, is allocated on the heap
    println!("{s}");

    let mut s = s; // it can be mutable
    s.push_str(", world!");
    println!("{s}");
}
```

In the case of a string literal it is known the content at compile time, so the text is hardcoded directly in the the executable and this is only possible because of its immutability.
With the `String` type, in order to support a mutable, growable text is needed to allocate an amount of memory on the *heap*, This means:
* The memory must be requested from the memory allocator at runtime (allocate).
* It is needed a way of returning this memory to the allocator when it is done with the `String` (free).

The first part is done on the code (`String::from(s)`), the second in Rust the memory is automatically returned to the allocator once the variable that owns the memory is out of scope.
```rust
fn main() {
    { // 's' not available because is not declared
        let s = String::from("hello");
        // do stuff with 's'
    } // 's' no longuer available because is out of scope
}
```

When a variable is out of scope, Rust calls the function `drop`. This function is called to the variable at the closing curly bracket.

## Variables and Data Interaction with Move
Multiple variables can interact with the same value in different way in Rust.
```rust
fn main() {
    // deep copy
    let x = 5; // giving value to x
    let y = x; // copying the value of x to y
    // now whe have two variable in two memory spaces on the stack
    
    // almost shallow copy | it is a move since 's1' will no longer be available
    let s1 = String::from("hello"); // a reference to the heap allocation is been stored on the
    // stack (pointer, length and capacity)
    let s2 = s1; // the pointer, length and capacity are been copyed to s2, but the content on the
    // heap remais the same

    // to avoid double free error, 's1' is no longer available
}
```

On the previous code it is possible to see that the behavior of Move is different from variable allocated on the stack and allocated on the heap.
The problems here is: when both `s1` and `s2` goes out of scope if could occur a *double free* error, the `drop` function would be called to the same heap memory allocated. Rust ensuure memory safty by making the `s1` variable no longer valid.
The completly copy on the `x` and `y` is called a *deep copy*, it the `s1` continued to be available would be a *shallow copy*. Since `s1` become invalid it is a *move*. `s1` moved to `s2`.

## Scope and Assingnment
When assigning a new value to an existing variable, Rust will automaticaly call `drop` and free up the space originally occupyed.
```rust
fn main() {
    let mut st = String::from("hello");
    st = String::from("ahoy");
    println!("{st}, world!")
    // the reassign will automatically free the previous memory to allocate and store a new one
}
```

## Variables and Data Interaction with Clone
To *deep copy* the heap data of a `String` it is possible using the method `clone`.
```rust
fn main() {
    let st1 = String::from("hello");
    let st2 = st1.clone();
    println!("st1 = {st1} st2 = {st2}");
}
```

## Ownership and Functions
Pass a variable to a function works the same way as passing to another variable, by copy or move.
```rust
fn main() {
    let str = String::from("hello");
    takes_ownership(str); // str move to function, so is no longer valid here
    // str is no longer available
    
    let x = 5;
    makes_copy(x); // just copy the value into the function
}

fn takes_ownership(s: String) {
    println!("{s}");
}
fn makes_copy(i: i32) {
    println!("{i}");
}
```

## Return Values and Scope
Returning values can also transfer ownership.
```rust
fn main() {
    let str1 = gives_ownership(); // created inside function and ownership given to 'str1'.
    let str2 = String::from("hello");
    let str3 = takes_and_gives_back(str2); // takes ownership from `str2` and make it invalid.
    // gives the ownership to 'str3'

    println!("str1 = {str1}; str3 = {str3}");
}

fn gives_ownership() -> String {
    String::from("hello")
}
fn takes_and_gives_back(s: String) -> String {
    s
}
```
When a variable that includes data on the heap goes out of scope the data is freed unless ownership is moved to another variable.

### Multiple Returns as Tuples
```rust
fn main() {
    let s1 = String::from("h1");
    let (s2, len) = calculate_length(s1); // takes ownership from s1 and gives to s2
    println!("String: {s2} Length: {len}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
```

## References and Borrowing
The issue with the previous code is that we still have to return the `String` in order to use it after is passed to the function. To solve this it is possible to pass a *reference* to the variable. A *reference* is like a pointer to the variable. This way the function will access the data stored in the address buut will not change its ownership.
```rust
fn main() {
    let s1 = String::from("h1");
    let len = calculate_length(&s1);
    println!("String: {s1} Length: {len}");
}
fn calculate_length(s: &str) -> usize {
    s.len()
}
```
The action of creating a reference is called *borrowing*. In this case is not possible to make changes in what is borrowed. References are immutable by default. In order to change must use a mutable reference.
```rust
fn main() {
    let mut s = String::from("mut");
    change(&mut s);
    println!("String: {s}");
}
fn change(s: &mut String) {
    s.push_str(" reference");
}
```

Mutable reference have a big restriction: if a value has a mutable reference to it, it can not have another reference.
```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;
    // it will give a compile error

    println!("{r1}, {r2}");
}
```
This is done so there is no data race happening. No scenario where two references try to change the same value at the same time. A data race heppens when:
* Two or more pointer access the same data at the same time.
* At least one pointer is being used to write to the data.
* There's no mechanism being used to synchronize access to the data.
It is possible to allow multiple mutable references, but not at the same scope.
```rust
fn main() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope and allows to create a new mutable reference
    let r2 = &mut s;
}
```

The same restriction is applied to mutable and non-mutable references to the same data.
```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{r1}, {r2}, and {r3}");
}
```

### Important
The reference's scope starts when it is declared and ends after the last time it is used. So this next code will run:
```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} {r2}"); // end of scope for 'r1' and 'r2'
    
    let r3 = &mut s; // no problem
    println!("{r3}");
}
```

### Dangling References
In a language with pointers, it's easy to erroneously create a *dangling pointer* - a pointer the references a location in memory that may be given to to something else - by freeing a memory while preserving a pointer to the memory. In Rust the compiler guarantees that reference will never be dangling references.
If there is a reference to some data, the compiler will guarantee that the data will not go out of scope before the reference to the data does.
```rust
fn main() {
    let reference_to_nothing = dangle(); // compile error
}
fn dangle() -> &String { // returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // the reference to that String is returned
} // but here the s String goes out of scope and is dropped
```

### The Rules of References
* Ate any given time, you can have either one mutable reference or multiple immutable references.
* References must always be valid.

## Slices
Slices let you reference a contiguous sequence os element in a collection (tuples, arrays). Since is a type of reference, it does not habe ownership.
Solve a problem of get first word without slices:
```rust
fn main() {
    let mut s = String::from("Hello World!");
    let index = first_word(&s);

    s.clear(); // empties the string making it equal to ""
    // trying to get the first word from 's' would cause an error
    // because the content of 's' changed.
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // converting to bytes to go through each element

    for (i, &item) in bytes.iter().enumerate() { // creates an iterator to go through the bytes and
        // enumerate to get the index
        if item == b' ' { // search the byte that is the first space
            return i; // return the index
        }
    }
    s.len() // if there is no space, return the index for the entire string
}
```

### String Slices
A *string slice* is a reference to a contiguous sequence of element of a `String`.
```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5]; // "hello"
    let world = &s[6..11]; // "world"
}
```
The syntax is `[start_index..end_index]` where `start_index` is the first position and `end_index` is the desired position plus one.
So, in the case of `let world = &s[6..11];`, would be a slice that contains a pointer to the byte of index `6` and a length of `5` (11-6).
If you want to start at index 0, you can drop the `start_idex`.
```rust
    let hello = &s[..5];
```
With the same logic, to get to the last byte on the `String`, just drop the `end_index`.
```rust
    let world = &s[6..];
```
To take a slice from all the `String`, drop both.
```rust
    let slice = &s[..];
```

> Note: String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.

Now back to the first word problem, but with slice:
```rust
fn main() {
    let mut s = String::from("Hello World!");
    let word = first_word_slice(&s);
    println!("First word: {word}");
}
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
```

### String Literals as Slices
String literals are stored inside the binary, and that is what slices are.
```rust
    let s = "Hello world!";
```
The type of `s` is `&str`: it's a slice pointing to that specific point of the binary. This is also why string literals are immutable, `&str` is a immutable reference.

### String Slices as Parameters
The fact that it is possible take slices from String leads us to an improvemnt on the first word problem:
```rust
fn main() {
    let s = String::from("Hello World!"); // it does not need to me mutable
    let word = first_word_slice(&s);
    println!("First word: {word}");
}
fn first_word_slice(s: &str) -> &str { // take a slice as a parameter
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s // since it is already a slice, return 's' entirely
}
```
This flexibility takes advantage of *deref coercions* (it can convert a `&String` to a `&str` because `String` implements `Deref` trait to return `&str`).
So all of this is possible:
```rust
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

### Other Slices
The same way it is possible to refer to a part of a string, it is possible to refer to a part of an array too.
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]); // true
}
```

# Struct
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("username"),
        email: String::from("user@name.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("user1@name.com");
    println!("Email: {}", user1.email);
}
```
In order to change a field on a structure the entire `User` instance must be mutable. Rust does not allow only some fields being mutable.

If a `build_user` function to return a `User` has the parameters with the same name as the struct fields it is possible to use the *Field Init Shortcut*.
```rust
// without field init shortcut
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```

```rust
// with field init shortcut
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

## Creating Instances from Other Instances With Struct Update Syntax
```rust
// without struct update syntax
fn main() {
    user1 = User {
        ...
    }

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("user2@name.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

```rust
// with struct update syntax
fn main() {
    user1 = User {
        ...
    }

    let user2 = User {
        email: String::from("user2@name.com"),
        ..user1
    };
}
```
The `..` specifies that the remainig fields not explicitly changed have the same values as the fields in the given instance. Is it is automaticaly passing the fields as arguments.
The `..user1` must come last. This operation will move the data from `user1` to `user2`, witch means that `user1` will no longuer be available. This happens because the the `String` value in `username` of `user1` was moved. It it was given to the `username` field of `user2` a new value, `user1` would still be valid, since the `sign_in_count` would not be moved but copyed.
Curiously the `user1.email` is istill available since it was not moved.
```rust
fn main() {
    user1 = User {
        ...
    }

    let user2 = User {
        email: String::from("user2@name.com"),
        ..user1
    };

    println!("user1 email: {}\nuser2 email: {}", user1.email, user2.email); // valid
}
```
### Important
The use of `String` type on the `User` struct is a choice so the data is owned by the structure and the data to be valid as long as the entire struct is valid. It wouldn't work is instead of `String` is a literal string (`&str`), this would be possible using *lifetime*, ensuring that the data referenced by a struct is valid for as long the struct is valid too.

## Tuple Struct
Have the added meaning that struct provides but without named fields, have just the type of the fields. Usefull to give name to tuples and distinguish one from another.
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

This way `black` and `origin` have the same values but are different types. In order to destructure a tuple struct is necessary to name the type. To access the values inside is the same way as the tuples.
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("point: {}, {}, {}", origin.0, origin.1, origin.2);
    let Color(r, g, b) = black;
    println!("color: {}, {}, {}", r, g, b);
}
```

## Unit-Like Struct
Those are structs that have no fields. Behave similarly to `()`. Can be useful when implementing a trait on some type but don't have any data to store in the type itself.
```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```
Imagine that later we’ll implement behavior for this type such that every instance of AlwaysEqual is always equal to every instance of any other type, perhaps to have a known result for testing purposes. We wouldn’t need any data to implement that behavior.

## Example - Rectangle
```rust
struct Rectangle {
    width: u32,
    heigth: u32,
}

fn area(rect: &Rectangle) -> u32 { // & to borrow, not to take ownership
                                // keeping the struct valid after
    rect.heigth * rect.width
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        heigth: 50,
    };

    println!("Area is: {}", area(&rect1));
}
```

If we try to print directly the `Rectangle` variable (`println!("{rect1}")`) it would get this compile error: `Rectangle` doesn't implement `std::fmt::Display`. The `println!` macro can do many kinds of formatting an the curly brackets tell `println!` to use `Display`, an implementation that the basic types have.
Along side with the error message if we try to print directly `rect1` have this message: note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead. Instead of implementing the `Display` for `Rectangle` (latter for this), we will try this *derived trait*.
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

fn area(rect: &Rectangle) -> u32 { // & to borrow, not to take ownership
                                // keeping the struct valid after
    rect.heigth * rect.width
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        heigth: 50,
    };

    println!("Rectangle: {rect1:?}");
    println!("Area is: {}", area(&rect1));
}
```
Putting the specifier `:?` inside the curly brackets tells `println!` to use an output format called `Debug`. Only this is not enough because `Rectangle` does not implement `Debug`. To make the functionality of printing debug information is added the outer attribute `#[derive(Debug)]` before the struct definition.
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

fn area(rect: &Rectangle) -> u32 { // & to borrow, not to take ownership
                                // keeping the struct valid after
    rect.heigth * rect.width
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        heigth: 50,
    };

    println!("Rectangle: {rect1:?}");
    println!("Area is: {}", area(&rect1));
}
```
For big structs we can use `{:#?}` instead of `{:?}`. It prints on a more structured way.
Another way to print using the `Debug` format is wwith the macro `dbg!`. The difference for the `println!` is that it takes ownership and it show more details about the file and the runnig file and the line number.
```rust
    dbg!(&rect1); // borrowing, if not if takes ownership
```

Since `dbg!` is an expression, it returns a value, so if can be used o differents parts of the code for debbuging.
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.heigth * rect.width
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // return and print the operation and result
        heigth: 50,
    };

    println!("Rectangle: {rect1:#?}");
    println!("Area is: {}", area(&rect1));
    dbg!(&rect1);
}
```

## Method Syntax
Change the `area` function to me a method of `Rectangle`.
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

impl Rectangle { // implementation context for Rectangle
    fn area(&self) -> u32 { // '&self' is a shortcut for 'self: &self'
        self.width * self.heigth
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        heigth: 50,
    };

    println!("Rectangle: {rect1:#?}");
    println!("Area is: {}", rect1.area());
    dbg!(&rect1);
}
```
`&self` is a shortcut for `self: &Self`. The `Self` type is an alias for the type that `impl` block is for. Rust methods must have the `&self` as the first parameter. The use of the `&` is for the method borrow the `Self` instance, same logic from previous cases. Methods can take ownership of `Self`, borrow it  immutably or borrow mutably.
It is possible to name method with the same name as fields:
```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.heigth
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    println!("Width is bigger then 0? {}", rect1.width());
}
```

### Curiosity: Rust's Method Call Syntax (No `->` Operator)
This text explains why Rust does not have the `->` operator commonly found in C and C++, and what mechanism it uses instead.

#### The C/C++ Approach
In C and C++, two distinct operators are used for calling methods:
* The `.` operator is used to call a method on an object directly.
    * `object.method()`
* The `->` operator is used to call a method on a *pointer* to an object. It first dereferences the pointer and then calls the method.
    * `pointer->method()` is equivalent to `(*pointer).method()`

#### The Rust Approach: Automatic Referencing and Dereferencing
Rust simplifies this by using only the `.` operator for all method calls. It achieves this through a feature called *automatic referencing and dereferencing*.

* **How it Works**: When you write `object.method()`, the Rust compiler automatically adds the necessary symbols (`&`, `&mut`, or `*`) to `object` to match the signature of the method being called.

* **The Role of `self`**: This behavior is possible because every Rust method has a clear "receiver"—the first parameter, which is always a variation of `self`:
    * `self`: The method takes ownership of the object.
    * `&self`: The method takes an immutable reference (a borrow).
    * `&mut self`: The method takes a mutable reference (a mutable borrow).

    The compiler checks the method's definition and automatically applies the correct operation to the object you're calling the method on. For example, `p1.distance(&p2)` and `(&p1).distance(&p2)` are treated as the same if the `distance` method is defined to take `&self`.

* **Primary Benefit**: This feature makes Rust's strict ownership and borrowing rules much more *ergonomic* and user-friendly. It leads to cleaner code because the programmer doesn't need to manually write `(*object).method()` or `(&object).method()` just to satisfy the compiler.

## Methods With Multiple Params
```rust
struct Rectangle {
    width: u32,
    heigth: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.heigth
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.heigth > other.heigth
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        heigth: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        heigth: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        heigth: 45,
    };

    println!("rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

## Associated Functions
All functions defined within the `impl` block are called *associated functions*. We can define associated functions without the `&self` as a parameter (so it is not a method), because it don't need an instance of the struct to work with.
To call an associated function is used the `::` to do so, just as used in `String::from()`.
```rust
#[derive(Debug)]
impl Rectangle {
    fn square(size: u32) -> Self { // Self is an alias for Rectangle
        Self {
            width: size,
            heigth: size,
        }
    }
}
fn main() {
    let sq = Rectangle::square(50);
    dbg!(&sq);
}
```

## Multiple `impl` Blocks
It is a valid syntax and is just like it's all inside one single `impl` block.
```rust

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            heigth: size,
        }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.heigth > other.heigth
    }

    fn area(&self) -> u32 {
        self.width * self.heigth
    }
}
```

# Enums
## Defining Enums
Where structs gives a way to group data together, enums give a way of saying a value is one of a possible set of values. For example, we may want to say that `Rectangle` is one of a set of possible shapes that also include `Circle` and `Triangle`. To do this, Rust allows to encode these possibilities as an enum.

Say we need to work with IP addresses. We will only get two variations of IP addresses, version four and version six. We can *enumerate* all possible variants, which is where enumeration gets its name. An IP address can be version four or version six, but not both at the same time. This property makes the enum data structure appropriate. Since version four and version six are still IP addresses, they should be treated as the same type.
```rust
enum IpAddrKind {
    V4,
    V6,
}
```

## Enum Values
It is possible to create instances of each variant:
```rust
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}
```

Now `four` and `six` are the same type, `IpAddrKind`. So it is possible to create a function that accept both as parameter.
```rust
fn route(ip_kind: IpAddrKind) {}

fn main() {
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}
```

So if we want to bind the actual IP address with the `IpAddrKind` we could use struct, but it is morre concise to use this concept with the enum itself.
```rust
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}
```

Along side with attach data to each variant, each variant also becomes a function that constructs an instance of the enum. So `IpAddr::V4()` is a function call that takes a `String` as argument and returns an instance of `IpAddr`.

Another advantage of enums over struct is that each variant can have different types and amounts of associated types.
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
```

However, wanting to store IP addresses an encode witch kind they are is so common that the standard library has a definition we can use.
```rust
struct Ipv4Addr {
    // something
}
struct Ipv6Addr {
    // something
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```
Another example with a variety of types embedded:
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

It is also possible to define method to enums with `impl`:
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // something
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
```

## The Option Enum and Its Advantages Over Null Values
`Option` is a enum defined by the standard libraby. It encodes the scenario in witch a value could something or nothing. This functionality can prevent bugs that are extremely commom in other languages.
Rust does not have the `null` feature that other languages have (like `nil` in Go). I his 2009 presentation "Null References: THe Billion Dolar Mistake", Tony Hoare, the inventor of null, said this:
> I call it my billion-dollar mistake. At that time, I was designing the first comprehensive type system for references in an object-oriented language. My goal was to ensure that all use of references should be absolutely safe, with checking performed automatically by the compiler. But I couldn’t resist the temptation to put in a null reference, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.

The problem with null values is when you try to use a null value as a non-null value. You'll get an error. But the concept is valid: a null ias a value that is currently invalid or absent.

Instead of null, Rust have an enum that can encode the concept. This enum is `Option<T>`:
```rust
enum Option<T> {
    None,
    Some(T),
}
```

The `Option<T>` doesn't need to be included from the standard library, is already available and its variants too, `Some` and `None` can be directly used directly without `Option::` prefix. Some examples:
```rust
fn main() {
    let some_number = Some(5); // type is Option<i32>
    let some_char = Some('e'); // type is Option<char>

    let absent_number: Option<i32> = None;
}
```

For the use of `None` is required to specify the overall `Option` type, the compiler can't infer the type in this situation since it is `None`.

When there is a `Some` value is guaranted that a value is present and it is held within `Some`. With `None` is similar to null, no value available, but what is the advantage of having an `Option<T>` to `None`?

Because since `Option<T>` and `T` are different types, the compiler won't allow to use an `Option<T>` as if it were definitly a valid value. E.g.:
```rust
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y; // not allowed by the compiler
}
```

When we have a `i8` value, the compiler will ensure that is always a valid value removing the necessity fo checking for null value. Only when is an `Option<T>` type is necessary to check if it has a value and the compile will ensure that all the possibilities will be handled.

You have to convert `Option<T>` to `T` before performing `T` operations.

[enum `Option<T>` documentation](https://doc.rust-lang.org/std/option/enum.Option.html)

## The `match` Control Flow Construct
`match` allows to compare a value against a series of patterns and execute code based on those patterns matches. The compiler confirms that all possible cases are handled.

Values go through each pattern in a `match`, and at the first pattern the value "fits", it falls into the associated code block. E.g.:
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel=> 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

The code assiciated with the `match` arm is an expression (it returns something) and the value returned is the value that gets returned by the `match` expression.

### Patters That Bind to Values
Match arms can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants.
```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel=> 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:#?}");
            25
        },
    }
}
```

When a `Coin::Quarter` matches, the `state` variable will be bind to the value of that quarter's state. Then we can use `state` in the code for the arm. E.g.:
```rust
fn main() {
    let c = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("Value in cents: {c}");
}
```

### Matching with `Option<T>`
Handling with `match` the `Some` and the `None` variants of `Option`.
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1), // the 'i' binds to the value in Some
        None => None,
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five); // six = Some(6)
    let none = plus_one(None); // none = None
}
```

The `plus_one` is an implementation of a method for the `Option` enum called `map`. Using it would do a more short version of the code.
```rust
fn main() {
    let five = Some(5);
    let six = five.map(|i| i + 1); // If there is a valid value, it will return with the defined change 'i + 1',
    //|i| is the declaration of the variable.
    assert_eq!(six, Some(6));
}
```

### Matches are Exhaustive
The compiler will make sure that you covered all possibilities.
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
} // Compile error: Pattern `None` not covered
```

### Catch-All Pattern and the `_` Placeholder
Catch-all is for all the non explicitly patterns declared.
```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_hat(),
        7 => remove_hat(),
        other => move_player(other),
    }
}

fn add_hat() {}
fn remove_hat() {}
fn move_player(num_spaces: u8) {}
```

It will bind the `dice_roll` value to the `other` variable (if its not 3 or 7) and then pass as parameter to the `move_player` function. All the different possibilities for the `dice_roll` was not explicitly covered, but it will compile because of the catch-all left arm.

If is needed a catch-all but don't want to use the value use `_`. This is a special pattern that match any value and does not bind to it.
```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_hat(),
        7 => remove_hat(),
        _ => reroll(),
    }
}

fn add_hat() {}
fn remove_hat() {}
fn reroll() {}
```

To make nothing happen use the `()` (called *unit*, an empty return).
```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_hat(),
        7 => remove_hat(),
        _ => (),
    }
}

fn add_hat() {}
fn remove_hat() {}
```

## Concise Control Flow with `if let` and `let else`
The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest.
```rust
    let config_max = Some(3u8); // number 3 as an u8
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
```

Using the `if let` syntax:
```rust
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
```

The `if let` takes a pattern and an expression separated by an equal sign. It works the same way as a `match` where the expression is given to the `match` and the pattern is its first arm. The block of code only runs if the value matchs the pattern.

Using `if let` is less typing, but also loses the exhaustive checking of `match`.

We can include an `else` with the `if let` statement. The block of that goes with the `else` is the same as it would go with the `_` case in `match`.
```rust
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        println!("Another code");
    }
```

## Staying on the "Happy Path" with the `let...else `
The common pattern is to perform some computation when a value is present and return a default value otherwise.
```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska=> year >= 1959,
            // --snip--
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel=> 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:#?}");
            25
        },
    }
}
```

Then whe might use `it let` to match the type of the coin, introducing a `state` variable within the body of the condition. If we want to print something related to the year:
```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
```

This get the job done, but it has pushed the job to the body of `if let` statement. We could also take advantage of the fact that expressions produce a value either to produce the `state` from the `if let` or return early: (You could do something similar with `match`)
```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

It is good, but to make this common pattern nicer to express, Rust has `let...else`. This syntax takes a pattern on the left side and and expression on the right, very similar to `if let`, but it does not have an `if` branch, only an `else`. If the pattern matches, it will bind the value from the pattern in the outer scope. If does not match, the program will flow into the `else` arm witch must `return` from the function.
```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

If you have a situation in which your program has logic that is too verbose to express using a `match`, remember that `if let` and `let...else` are in your Rust toolbox as well.

# Managing Growing Projects with Packages, Crates and Modules
Rust has a number of features that allows to manage code organization. These features, sometimes collectively called as the *module system*, include:
* **Packages:** A Cargo feature that lets you build, test and share crates
* **Crates:** A tree of modules that produces a library or executable
* **Modules and use:** Let you control the organization, scope and privacy of paths
* **Paths:** A way of naming an item, such as a struct, function or module

## Packages and Crates
A *crate* is the smallest amount of code that the Rust compiler considers at a time. Crates can contain modules, and modules may be defined in other files that get compiled with the crate.

A crate can come in one of two forms: a binary crate or a libraby crate. *Binary crates* are programs compiled into a executable that you can run. Each must have a function called `main` that defines what happens when the executable runs.

*Library crates* don't have a `main` function, and they don't compile to an executable. They define functionality intended to be shared with multiple projects. For example, the `rand` crate used before.

The *crate root* is a source file that the Rust compiler starts from and makes up the root module of your crate.

A *package* is a bundle of one or more crates that provides a set of functionality. A package contains a *Cargo.toml* file that describes how to build those crates.

A package can contain as many binary crates as you like, but at most only one library crate. A package must contain at least one crate, whether that's a binary or library crate.

After running `cargo new new-project` is created a `Cargo.toml` file and the `src/main.rs` directory and file. `Cargo.toml` defines that is a package, inside of it there is no mention of `src/main.rs`. Cargo follows the convention that `src/main.rs` is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package directory contains `src/lib.rs`, the package contains a library crate with the same name as the package and `src/lib.rs` is its crate root. Cargo passes the crate root to `rustc` to build the library or binary.

A package can have multiple binary crates by placing files in the `src/bin` directory: each file will be a separeted binary crate.

## Defining Modules to Control Scope and Privacy
### Modules Cheat Sheet
Here is a quick reference on how modules, paths, the `use` keyword and the `pub` keyword work in the compiler, and how most developers organize their code.
* **Start from the crate root**: When compiling a crate, the compiler first looks in the crate root file (usualy *src/lib.rs* for a library crate or *src/main.rs* for a binary crate) for code to compile.
* **Declaring modules**: In the crate root file, you can declare new modules; say you declare a "garden" module with `mod garden;`. The compiler will look for the module's code in these places:
    * Inline, within curly brackets that replace the semicolon following the `mod garden`
    * In the file *src/garden.rs*
    * In the file *src/garden/mod.rs*
* **Paths to code in modules**: Once a module is part of your crate, you can refer to code in that module from anywhere else in the same crate, as long as the privacy rules allow, using the path to the code. For example, an `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.
* **Private vs. Public**: Code within a module is private from its parent module by default. To make a module public, declare it with `pub mod` instead of `mod`. To make itens within a public module public as well, use pub before their declaration.
* **The `use` keyword**: Within a scope, the `use` keyword creates a shortcut to items to reduce repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use crate::garden::vegetables::Asparagus;` and from then on you only need to write `Asparagus` to make use of that type in the scope.

A binary crate called *backyard*:
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs

The use:
```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
```

The `pub mod garden;` line tells the compiler to include the code it finds in *src/garden.rs* which is:
```rust
pub mod vegetables;
```

Here `pub mod vegetables;` means the code in *src/garden/vegetables.rs* is included too. That code is:
```rust
#[derive(Debug)]
pub struct Asparagus {}
```

Now let’s get into the details of these rules and demonstrate them in action!

### Grouping Related Code in Modules
*Modules* let us organize code within a crate for readability and easy reuse. Also allow us to control the *privacy* of items because code within a module is private by default.

Crate a new library named `restaurant` by running `cargo new restaurant --lib`. Then edit the file *src/lib.rs*:
```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
```

A module is defined with the `mod` keyword followed by the name of the module. Inside modules we can place other modules, in this case are `hosting` and `serving`. Modules can also hold definitions for other itens, such as structs, enums, constants, traits and functions.

We can group related definitions together and name why they are related.

*src/main.rs* and *src/lib.rs* are crate roots. The reason for their name is that the contents of either of these two files form a module name `crate` at the root of the crate's module structure, known as the *module tree*:

crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

## Paths for Referring to an Item in the Module Tree
To show Rust where to find an item in a module tree, is used a path the same way as navigating a filesystem. A path can take two forms:
* **Absolute path**: Is the full path starting from a crate root; for code from a external crate, the absolute path begins with the crate name and for code from the current crate, it starts with the literal `crate`.
* **Relative path**: Starts from the current module and use `self`, `super` or an identifier in the current module.

Both *absolute* and *relative* path are followed by one or more identifiers separated by double colons (`::`).

## Exposing Paths with the `pub` Keyword
Items in a parent module can't use the private items inside a child module, but a child module can use all items in their ancestor module. This happens because a child module is create wrapped inside the context of the parent.

Rust gives the option to expose inner parts of a module with the `pub` keyword put as a prefix on a definition.

So in the file *src/lib.rs* we can do:
```rust
mod front_of_house { // no need to be public because is defined in the same module as eat_at_restaurant
    pub mod hosting { // public to be accessed by eat_at_restaurant
        pub fn add_to_waitlist() {} // public to be accessed by eat_at_restaurant
    }
}

fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

### Summary: Rust Packages with a Binary and Library
This text outlines the best practice for structuring a Rust package that contains both a runnable binary (`src/main.rs`) and a reusable library (`src/lib.rs`).

* **Core Principle**: The binary crate should be a minimal wrapper that calls the code defined in the library crate. All the main logic, modules, and functionality should reside in `src/lib.rs`.

* **Benefit of this Structure**: This pattern maximizes code reuse. Other projects can depend on your package as a library and use its core functionality, while you still provide a default executable.

* **Implementation**: The binary crate (`src/main.rs`) acts as a client of its own library crate. It can only access the *public API* exposed by the library, just like any external crate would.

* **API Design Advantage**: This practice forces you to design a good, clean public API. By being both the author and the first client of your own library, you immediately experience how easy (or difficult) it is to use, which leads to better API design.

## Starting Relative Paths with `super`
We can construct relative paths that begin in the parent module, rather then the current module or the crate root by using `super` at the start of the path. This is like starting a filesystem path with the `..`.
```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

## Making Structs and Enums Public
We can also use the `pub` keyword to make structs and enums public, but there are a few extra details. If put the `pub` before a struct definition, the struct will be public but its fields will remain private.
```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change the bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // This next like wouldn't work because 'seasonal_fruit' is private
    // meal.seasonal_fruit = String::from("blueberries");
}
```

In contrast, if a enum is made public, all its variants are automatically public too.
```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

## Bringing Paths Into Scope With the `use` Keyword
A more convinient way to use items from other modules is to use the `use` keyword. With this keyword we can create a shortcut and include the item inside the current scope.
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

`use` only creates a the shortcut to the scope in with it is used.
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist(); // compile error: undefined crate or module `hosting`
    }
}
```

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
} 
```

### Idiomatic `use` Declarations in Rust
The text outlines the conventional best practices (idioms) for bringing items into scope with the `use` keyword in Rust. The convention differs for functions versus other items like structs and enums to enhance code clarity.

#### For Functions
It's idiomatic to bring the function's *parent module* into scope, not the function itself.
* **Practice**: Instead of `use a::b::c_function;`, you should write `use a::b;`.
* **Usage**: You then call the function with the module name prefixed, like `b::c_function();`.
* **Reasoning**: This makes it obvious that the function being called is not defined locally within the current scope, which prevents ambiguity about the function's origin.

#### For Structs, Enums, and Other Items
In contrast to functions, it's idiomatic to bring in *structs*, *enums*, and other similar items using their full path.
* **Practice**: Write `use std::collections::HashMap;`.
* **Usage**: You can then refer to the item directly, like `let mut map = HashMap::new();`.
* **Reasoning**: This is the standard convention that makes using these types more direct and less verbose at the call site.

### Providing New Names with the `as` Keyword
```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### Re-exporting Names with the `pub use`
When bringing a name into scope with `use`, the name is private to the current scope. For code from outside be able to access the import we can combine `pub` and `use`. This is called *re-exporting*.
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

Before this change, external code would use the `add_to_waitlist()` function by calling the `eat_at_restaurant()` function, an indirect way. Now it is possible to call directly.

### Using Nested Paths to Clean Up Large `use` Lists
If using many intems defined on the same crate or same module, listing then line by line can take a lot of vertical space.
```rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
```

It is possible to use nested paths to bring the same items into scope in one line.
```rust
// --snip--
use std::{cmg::Ordering, io};
// --snip--
```

It is very useful when combining two `use` statements that share a subpath.
```rust
// --snip--
use std::io;
use std::io::Write;
// --snip--
```

Turns into:
```rust
use std::io::{self, Write};
```

### The Glob Operator
If we want to bring all public items defined in a path into scope, we can use `*`.
```rust
use std::collections::*;
```

It is often used when testing.

## Separationg Modules into Different Files
Code on *src/lib.rs*:
```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

```

Code on *src/front_of_house.rs*:
```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

From the `mod front_of_house;` statement at *src/lib.rs* the compiler will look to a file named *front_of_house.rs*. Now we'll extract the `hosting` to its own file.
New code on *src/front_of_house.rs*:
```rust
pub mod hosting;
```

Code in file *src/front_of_house/hosting.rs*;
```rust
pub fn add_to_waitlist() {
    println!("Added!");
}
```

If we instead put *hosting.rs* in the *src* directory, the compiler would expect the *hosting.rs* code to be in a `hosting` module declared in the crate root and not declared as a child of the `front_of_house` module.

To use on a *src/main.rs* would be:
```rust
use restaurant::front_of_house;

fn main() {
    front_of_house::hosting::add_to_waitlist();
}
```

# Common Collections
Most other data types represent one specific value, but *collections* can contain multiple values. Rust's standard includes a number of useful *collections*. Tree of those are:
* **Vector**: Allows to store a variable number of values next to each other.
* **String**: Is a collection of characters.
* **Hash Map**: Allows to associate a value with a specific key. It's a particular implementation of the general data structure called *map*.

[Collections Documentation](https://doc.rust-lang.org/std/collections/index.html)

## Storing Lists of Values with Vectors
Vectors allows you to store more then one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type. They are useful when you have a list of items, such as the lines of text in a file.

### Creating a New Vector
```rust
let v: Vec<i32> = Vec::new();
```

Since is not beeing inserted any value, is necessary the type annotation `Vec<i32>`. Rust conveniently provides the `vec!` macro, which will create a new vector that hold the values you give.
```rust
let v = vec![1, 2, 3];
```

### Updating a Vector
To add elements to a vector we can use the `push` method.
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    dbg!(&v);

    v.push(4);
    v.push(5);
    dbg!(&v);
}
```

### Reading elements of Vectors
There are two ways to reference a value stored in a vector: via indexing or with the `get` method.
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // getting a reference to the third value
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2); // 'get' returns an 'Option<&T>' value
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    };
}
```

When trying to read an index out of range: (OOB - Out Of Bounds)
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100]; // panics and oob error
    let does_not_exist = v.get(100); // return `None` without panicking
}
```

It is not possible to have a mutable and immutable references to the contents of the same object (in this case a vector).
```rust
fn main() {
    let mut ve = vec![1, 2, 3, 4, 5];
    let first = &ve[0];

    ve.push(6); // compile error

    println!("First: {first}");
}
```

This error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.

### Iterating Over the Values in a Vector
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("{i}");
    }
}
```

We can also iterate on mutable references.
```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        *i += 50;
    }
    dbg!(&v);
}
```

### Using an Enum to Store Multiple Types
Vectors can only store one type, so to store multiple types we can use enums.
```rust
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    dbg!(&row);
}
```

Rust needs to know what types will be in the vector at compile time to know how much memory is going to be allocated on the heap. Using an `enum` plus a `match` expression means that Rust will ensure at compile time that every possible case is handled.

### Dropping a Vector Drops its Elements
Like `struct`, a vector is freed when it goes out of scope.
```rust
{
    let v = vec![1, 2, 3];
} // v goes OOS and is freed
```

## Storing UTF-8 Encoded Text with String
### Creating a New String
```rust
    let mut s = String::new(); // empty String
```

To make some initial data into a `String` we have the method `to_string` which is available to all types that implement the `Display` trait.
```rust
fn main() {
    let data = "some data";
    let s1 = data.to_string();

    let s2 = "more data".to_string();
}
```

And there is also the function `String::from()`.
```rust
    let s = String::from("inital content");
```

Since `String` are UTF-8 encoded, we can include any properly encoded data in them.
```rust
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
```

### Updating a String
Like `Vec<T>`, strings can grow in size and its content can change. You can conveniently use the `+` operator or the `format!` macro to concatenate `String` values.

#### Appending to a String with `push_str` and `push`
```rust
fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // it takes a string slice, so it dont take ownership
    println!("s1 + {s2} = {s1}"); // using s2 after
}
```

The `push` method takes a single character as parameter and concatenates.
```rust
fn main() {
    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");
}
```

#### Concatenating with the `+` Operator or the `format!` Macro
```rust
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 was moved, s2 just referenced
    println!("s3: {s3}");
}
```

It is not possible to add two `String` together, only a `&str` to a `String` and that is what must be done with the `+` operator. `&s2` is actualy a `&String` type, not a `&str`, but the compiler can use a *deref coercion*, which here turns `&str` into `&str[..]`. This behavior happens because the `add` function for the `+` operator is like this:
```rust
fn add(self, s: &str) -> String {
```

So it takes ownership of `self`, in this case of `s1`, making `s1` no longer valid after the plus operation.

For combining strings in more complicated way there is the `format!` macro. Works like `println!` but instead of printing, returns a `String`.
```rust
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s: {s}"); // tic-tac-toe
}
```

### Indexing into String
It is not possible to access individual characters in a string by indexing like `s[0]`. To understand, lets see how they are internaly represented.

#### Internal Representation
A `String` is a wrapper over a `Vec<u8>`. If we create a `String` as follow:
```rust
    let hello = String::from("Hello");
```

`len` would be 4. It takes 1 byte to store each character on that string. But when creating this:
```rust
    let hello = String::from("Здравствуйте");
```

`len` should be 12 (since there is 12 characters) but is actualy 24, that's the number of bytes necessary to encode in UTF-8. This happens because each Unicode scalar value in that string takes 2 bytes of storage. Therefore, an index into the string's bytes will not always correlate to a valid Unicode scalar.

To avoid returning an unexpected value and causing bugs that might not be discovered immediately, Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.

### Slicing Strings
Indexing into a string if often a bad idea because is not clear what will return. If is really needed, instead of using `[]` with a single number, use it with a range.
```rust
fn main() {
    let hello = String::from("Здравствуйте");
    let s = &hello[0..4]; // since it is 2 bytes per char, it works
    println!("{s}");
}
```

If we were to try to slice only part of a character’s bytes with something like `&hello[0..1]`, Rust would panic at runtime in the same way as if an invalid index were accessed in a vector.

### Methods for Iterating Over Strings
```rust
fn main() {
    for c in "Зд".chars() {
        println!("{c}"); // each char
    }
    for b in "Зд".bytes() {
        println!("{b}"); // each byte
    }
}
```

## Storing Keys with Associated Values in Hash Maps
The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V` using a *hashing funciton*.

### Creating a New Hash Map
```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
```

### Accessing Values in a Hash Map
```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name) // 'get' returns an 'Option<&V>', if there's no value,
        // it will return 'None'
        .copied() // This handles the 'Option' returned to get an 'Option<i32>' rather then an
        // 'Option<&V>' (getting a copy, not a reference)
        .unwrap_or(0); // return 0 if the does not have an entry for the key

    println!("Score of team {team_name}: {score}");
}
```

Iterating:
```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores { // reference so the value is not moved
        println!("{key}: {value}");
    }
}
```

### Hash Maps and Ownership
For types that implement the `Copy` trait, like `i32`, the value will be copyed into the hash map. For owned values like `String`, the values will be moved and the hash map will own the values.
```rust
use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let value_name = String::from("Bluish Green");

    let mut map = HashMap::new();
    map.insert(field_name, value_name);
    // now 'field_name' and 'value_name' are invalid
    dbg!(&map);
}
```

If we insert references to values into the hash map, the values won’t be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid.

### Updating a Hash Map
#### Overwriting a Value
If we insert a key and a value into a hash map and insert the same key but a different value, the value will be overwrite.
```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores.insert(String::from("Blue"), 25);
    dbg!(&scores);
}
```

#### Adding a Key and Value Only If a Key Isn’t Present
It's common to check whether a key already exists in the hash map to take the following action: if the key does exists, the existing value remains the same, if it doesn't insert the keyt and some value.
```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // create and insert 50
    scores.entry(String::from("Yellow")) // returns a enum 'Entry' represention a value that
        // exists or not
        .or_insert(50); // method of 'Entry' that returns a mutable reference to the value if
    // exists; if not, inserts the parameter as the new value for the given key

    // keeps the previous value (10)
    scores.entry(String::from("Blue"))
        .or_insert(50);
}
```

#### Updating a Value Based on the Old Value
```rust
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // 'or_insert' returns a mutable reference
        // allowing to change the value
        *count += 1;
    }

    println!("{map:?}"); // {"world": 2, "hello": 1, "wonderful": 1}
}
```

#### hashing Functions
By default, `HashMap` uses a hashing function called SipHash that can provide resistance to denial-of-service (DoS) attacks involving hash tables1. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it. If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher. A hasher is a type that implements the `BuildHasher` trait.

# Error Handling
## Recoverable Errors with `Result`
The `Result` enum is defined as having two variants: `Ok` and `Err`:
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

So if we try to read a file (which return a `Result`):
```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt"); // the return type of is a Result<T, E>
    // Here T is a std::fs::File and E is a std::io::Error

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file: {error:?}"),
    };
}
```

## Matching on Different Errors
Add a inner `match` expression.
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt"); 

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() { // the error returned is of type 'io::Errror'. It has a
            // method called 'kind()' that return a io::ErrorKind
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {e:?}")
            },
            _ => panic!("Problem opening file: {error:?}"),
        }
    };
}
```

### Shortcuts for Panic on Error: `unwrap` and `expect`
The `unwrap` method is a shortcut implemented just like the `match` expression wrote previously. If the `Result` is the `Err` variant, `unwrap` will call `panic!`.
```rust
use std::fs::File;

fn main() {
    let greeting = File::open("hello.txt").unwrap(); // if is an Err, call panic!
}
```

Similarly, `expect` calls `panic!` but let us choose the error message too.
```rust
use std::fs::File;

fn main() {
    let greeting = File::open("hello.txt")
        .expect("hello.txt should be included"); // if is an Err, call panic!
}
```

### Propagating Errors
When a function implementation calls something that might fail, instead of handling the error, it is possible to return the error to the caller and let the caller handle the error. This is called *propagating error*.
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

### A shortcut for Propagating Errors: The `?` Operator
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;

    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

The `?` placed after a `Result` variable is defined to work in almost the same way as the `match` expresssion defined previously. It will define that if `Result` is `Ok` then will return the value inside of `Ok`. If it is an `Err`, the `Err` will be returned for the whole function, like using a `return`.

Error values that have the `?` operator called on the will go through the `from` function, defined in the `From` trait in the standard libraby which is used to convert types from one value to another. When the operator calls the `from` function, the error type defined received is converted into the error type defined in the return type of the current function.

It is a very useful operator that can eliminates a lot of boilerplate.
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

An even shorter way to do it with `fs::read_to_string` from the standard libraby:
```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

#### When the Operator `?` Can Be Used
The `?` can only be used in functions where the return types are compatible with the value the `?` is used on.

The `?` operator is only allowed to be used on a function that returns `Result`, `Option` or another type that implements `FromResidual`.

The behavior of the `?` operator when called on an `Option<T>` is similar to its behavior when called on a `Result<T, E>`: if the value is `None`, the `None` will be returned early from the function at that point. If the value is `Some`, the value inside the `Some` is the resultant value of the expression, and the function continues.
```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
    // lines return an iterator for the lines on the text
    // next calls the next line, in this case the first one
    // chars return an iterator for every character on the line
    // last return the last char
}
```

This is an `Option` because the first line can be empty.

The `main` function can return a `Result<(), E>`:
```rust
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}
```

The `Box<dyn Error>>` type is a *trait object*, not covered yet, but for now means "any kind of error".

The `main` function can return any type that implements the `std::process::Termination` trait, wich contains a function `report` that returns an `ExitCode`.

# Generic Types, Traits and Lifetimes
## Generic Data Types
### In Function Definition
Problem to get the largest value on a list:
```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for v in list {
        if v > largest {
            largest = v;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("Largest number: {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("Largest char: {result}");
}
```

This code above wouldn't still work because `T` is not restricted with the `PartialOrd` trait. More on that on the trait section.

### In Struct Definition
```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point{ x: 5, y: 10 };
    let float = Point{ x: 1.0, y: 4.0 };
    let integer_float = Point{ x: 5, y: 4.0 };
}
```

### In Enum Definition
We've already seen:
```rust
enum Option<T> {
    Some(T),
    None,
}
enum Result<T,E> {
    Ok(T),
    Err(E),
}
```

### In Method Definition
```rust
struct Point<T,U> {
    x: T,
    y: U,
}
impl<T,U> Point<T,U> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point{ x: 5, y: 10 }; 
    println!("p.x: {}", p.x);
}
```

By declaring `T` as a generic type after `impl`, Rust can identify that the type in the angle brackets in `Point` is a generic type rather than a concrete type. We could have chosen a different name for this generic parameter than the generic parameter declared in the struct definition, but using the same name is conventional.

It is possible specify constraints on generic types, implement a method to a specified type.
```rust
impl Point<f32, f32> { // only applies to T and U beeing f32
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

Generic types in a structure aren't always the same as those used in the same struct's method signature.
```rust
struct Point<X1,Y1> {
    x: X1,
    y: Y1,
}

impl<X1,Y1> Point<X1,Y1> {
    // get the y of another Point and mix with the current Point
    fn mixup<X2,Y2> (self, other: Point<X2,Y2>) -> Point<X1,Y2> {
        Point { x: self.x, y: other.y }
    }
}

fn main() {
    let p1 = Point{ x: 5, y: 10.4 };
    let p2 = Point{ x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}\np3.y = {}", p3.x, p3.y); // 5 'c'
}
```

### Performance of Code Using Generics
Generics does not affect the code performance. Rust accomplishes this by performing monomorphization of the code using generics at compile time.

*Monomorphization* is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. In this process, the compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.

## Traits: Defining Shared Behavior
A *trait* defines the functionality a particular type has and can share with other types. We can use it to define shared behavior in an abstract way.

> Traits are similar to a feature often called *interface* in other pragramming languages (like Go), but with some differences.

### Defining a Trait
A type's behavior consists of the methods we can call on that type. Different type share the same behavior if we can call the same methods in all of then. Trait definition are a way to group method signatures together to define a set of bahaviors necessary to accomplish some purpose.

For example, let’s say we have multiple structs that hold various kinds and amounts of text: a `NewsArticle` struct that holds a news story filed in a particular location and a `SocialPost` that can have, at most, 280 characters along with metadata that indicates whether it was a new post, a repost, or a reply to another post.

We want to make a media aggregator library crate named `aggregator` that can display summaries of data that might be stored in a `NewsArticle` or `SocialPost` instance. To do this, we need a summary from each type, and we’ll request that summary by calling a `summarize` method on an instance. Listing 10-12 shows the definition of a public `Summary` trait that expresses this behavior.
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

### Implementing a Trait on a Type
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}
impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{} : {}", self.username, self.content)
    }
}

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        repost: false,
    };
    
    println!("1 new post: {}", post.summarize());
}
```

We can't implement external traits on external types, only if either the trait or the type, or both, are local to our crate. For example, we can't implement the `Display` trait on `Vec<T>` within a package created by us.


### Default Implementation

