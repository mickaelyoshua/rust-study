fn main() {
    let s = "hello"; // this is a string literal, is immutable and allocated on the stack
    println!("{s}");

    let s = String::from(s); // this is a String type, is allocated on the heap
    println!("{s}");

    let mut s = s; // it can be mutable
    s.push_str(", world!");
    println!("{s}");

    // deep copy
    let x = 5; // giving value to x
    let y = x; // copying the value of x to y
    // now whe have two variable in two memory spaces on the stack
    println!("{x} {y}");
    
    // almost shallow copy | it is a move since 's1' will no longer be available
    let s1 = String::from("hello"); // a reference to the heap allocation is been stored on the
    // stack (pointer, length and capacity)
    let s2 = s1; // the pointer, length and capacity are been copyed to s2, but the content on the
    // heap remais the same
    println!("{s2}");

    // to avoid double free error, 's1' is no longer available
    

    // let mut st = String::from("hello");
    // st = String::from("ahoy");
    // println!("{st}, world!");
    // // the reassign will automatically free the previous memory to allocate and store a new one
    
    let st1 = String::from("hello");
    let st2 = st1.clone();
    println!("st1 = {st1} st2 = {st2}");


    let str = String::from("hello");
    takes_ownership(str); // str move to function, so is no longer valid here
    // str is no longer available
    
    let x = 5;
    makes_copy(x); // just copy the value into the function
    

    let str1 = gives_ownership(); // created inside function and ownership given to 'str1'.
    let str2 = String::from("hello");
    let str3 = takes_and_gives_back(str2); // takes ownership from `str2` and make it invalid.
    // gives the ownership to 'str3'

    println!("str1 = {str1}; str3 = {str3}");


    let s1 = String::from("h1");
    let len = calculate_length(&s1);
    println!("String: {s1} Length: {len}");

    let mut s = String::from("mut");
    change(&mut s);
    println!("String: {s}");


    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} {r2}"); // end of scope for 'r1' and 'r2'
    
    let r3 = &mut s; // no problem
    println!("{r3}");
}

fn takes_ownership(s: String) {
    println!("{s}");
}
fn makes_copy(i: i32) {
    println!("{i}");
}

fn gives_ownership() -> String {
    String::from("hello")
}
fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" reference");
}
