fn main() {
    let s = String::from("Hello World!");
    let word = first_word_slice(&s);
    println!("First word: {word}");

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2,3]);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes(); // converting to bytes to go through each element
//
//     for (i, &item) in bytes.iter().enumerate() { // creates an iterator to go through the bytes and
//         // enumerate to get the index
//         if item == b' ' { // search the byte that is the first space
//             return i; // return the index
//         }
//     }
//     s.len() // if there is no space, return the index for the entire string
// }

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}
