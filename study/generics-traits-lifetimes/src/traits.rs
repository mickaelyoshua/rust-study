pub mod tr {
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for v in list {
            if v > largest {
                largest = v;
            }
        }
        largest
    }

    pub trait Summary {
        fn summarize(&self) -> String;
    }
    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
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

    pub fn test() {
        let post = SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people"
            ),
            reply: false,
            repost: false,
        };
        
        println!("1 new post: {}", post.summarize());

        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("Largest number: {result}");

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&char_list);
        println!("Largest char: {result}");
    }
}
