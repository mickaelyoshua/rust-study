pub mod lt {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    struct ImportantExcept<'a> {
        part: &'a str,
    }

    pub fn test() {
        // let r;
        // let x = 5;
        // r = &x;
        // println!("r: {r}");


        let string1 = String::from("long string is long");
        {
            let string2 = String::from("xyz");
            let result = longest(&string1, &string2);
            println!("The longest string is '{result}'");
        }


        let novel = String::from("Call me Ishmael. Some year ago...");
        let first_sentence = novel.split(".").next().unwrap();
        let i = ImportantExcept {
            part: first_sentence,
        };

        println!("{}", i.part);
    }
}
