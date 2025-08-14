pub mod hashmap {
    use std::collections::HashMap;

    pub fn test() {
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
        
        println!("{scores:?}");

        let team_name = String::from("Blue");
        let score = scores.get(&team_name) // 'get' returns an 'Option<&V>', if there's no value,
            // it will return 'None'
            .copied() // This handles the 'Option' returned to get an 'Option<i32>' rather then an
            // 'Option<&V>' (getting a copy, not a reference)
            .unwrap_or(0); // return 0 if the does not have an entry for the key

        println!("Score of team {team_name}: {score}");

        for (key, value) in &scores { // reference so the value is not moved
            println!("{key}: {value}");
        }


        let field_name = String::from("Favorite color");
        let value_name = String::from("Bluish Green");

        let mut map = HashMap::new();
        map.insert(field_name, value_name);
        // now 'field_name' and 'value_name' are invalid
        dbg!(&map);

        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0); // 'or_insert' returns a mutable reference
            // allowing to change the value
            *count += 1;
        }

        println!("{map:?}");
    }
}
