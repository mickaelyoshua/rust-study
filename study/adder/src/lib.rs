pub mod guess {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if !(1..100).contains(&value) {
                panic!("Guess value must be between 1 and 100, got {value}.");
            }
            Guess { value }
        }
    }
}

pub mod ad {
    pub fn add(left: u64, right: u64) -> u64 {
        left + right
    }

    pub fn add_two(a: u64) -> u64 {
        a + 2
    }
}

pub mod rec {
    #[derive(Debug)]
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }
    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello")
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::guess::Guess;
    use crate::ad::{add, add_two};
    use crate::rec::Rectangle;

    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }

    #[test]
    #[should_panic(expected = "less then or equal to 100")]
    fn greather_then_100() {
        Guess::new(200);
    }

    #[test]
    fn greeting_contains_name() {
        let name = "Carol";
        let result = greeting(name);
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
    );
    }
    
    #[test]
    fn it_adds_two() {
        let result = add_two(6);
        assert_eq!(result, 8);
    }

    #[test]
    fn its_not_the_same_number() {
        let a = 5;
        let result = add_two(a);
        assert_ne!(a, result);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{
            width: 8,
            height: 7,
        };

        let smaller = Rectangle{
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle{
            width: 8,
            height: 7,
        };

        let smaller = Rectangle{
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

