pub mod st {
    pub fn test() {
        let data = "some data";
        let s1 = data.to_string();

        let s2 = "more data".to_string();

        println!("s1: {s1}\ns2: {s2}");

        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2); // it takes a string slice, so it dont take ownership
        println!("s1 + {s2} = {s1}"); // using s2 after
        
        let mut s = String::from("lo");
        s.push('l');
        println!("{s}");

        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // s1 was moved, s2 just referenced
        println!("s3: {s3}");

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}");
        println!("s: {s}"); // tic-tac-toe
        

        let hello = String::from("Здравствуйте");
        let s = &hello[0..4]; // since it is 2 bytes per char, it works
        println!("{s}");


        for c in "Зд".chars() {
            println!("{c}");
        }
        for b in "Зд".bytes() {
            println!("{b}");
        }
    }
}
