pub mod v {
    #[derive(Debug)]
    pub enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    pub fn test() {
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Float(10.12),
            SpreadsheetCell::Text(String::from("blue")),
        ];
        dbg!(&row);

        let mut v = vec![1, 2, 3, 4, 5];

        for i in &mut v {
            *i += 50;
        }
        dbg!(&v);

        let third: &i32 = &v[2]; // getting a reference to the third value
        println!("The third element is {third}");

        let third: Option<&i32> = v.get(2); // 'get' returns an 'Option<&T>' value
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element"),
        };
    }
}
