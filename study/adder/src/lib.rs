pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: u64) -> u64 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello")
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
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

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
