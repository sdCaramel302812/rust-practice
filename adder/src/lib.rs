#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn bug_can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub fn greeting(name: &str) -> String {
    String::from("hello")
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let large = Rectangle {
            width: 7,
            height: 5
        };
        let small = Rectangle {
            width: 1,
            height: 1
        };
        assert!(large.can_hold(&small));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let large = Rectangle {
            width: 7,
            height: 5
        };
        let small = Rectangle {
            width: 1,
            height: 1
        };
        assert!(!small.can_hold(&large));
    }

    #[test]
    #[ignore]
    fn larger_can_hold_smaller_with_bug() {
        let large = Rectangle {
            width: 7,
            height: 5
        };
        let small = Rectangle {
            width: 1,
            height: 1
        };
        assert!(large.bug_can_hold(&small));
    }

    #[test]
    fn it_add_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(5, add_two(2));
    }

    #[test]
    #[ignore]
    fn greeting_cotain_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
            "Greeting didn't contain name, value is \"{}\"",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")] // it panic message contains expected string, it pass
    fn guess_should_panic() {
        let guess = Guess::new(101);
    }

    #[test]
    fn test_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("result should equal to four"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {                   // it should be ignore
        // code that takes an hour to run
    }
}
