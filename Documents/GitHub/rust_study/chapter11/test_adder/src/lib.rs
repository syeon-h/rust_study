/*
    test functions typically perform three actions: 
        1. set up any needed data or state 
        2. run the code you want to test 
        3. assert that the results are what you expect 
    
    anatomy: test is a function that's annotated with the test attribute 
        attributes are metadata about pieces of Rust code (derive[Debug])  
        #[test] annotation indicates that this is a test function 
        might also have non-test functions in the tests module 

    test module is a regular module that follows the usual visibility rules 
        because the test module is an inner, need to bring the code under 
        test in the outer module into the scope using use super::*; 

    assert_ne! macro will pass if the two given values are not equal 
        most useful for cases when we are not sure what a value WILL be, 
        but we know what the value definitely SHOULD NOT be 
        Ex. function that is guaranteed to change its input in some way 

    assert_eq! and assert_ne! macros use the operations == and !=, respectively 
        when fail, these print their arguments using debug formatting, meaning 
        the value being compared must implement the PartialEq and Debug traits 

    should_panic test passes if the code inside the function panics 
        test would pass even if the test panics for a different reason 
        thus, to make more pecise, add an optional expected param (harness)
        which make sure that the failure message contains the provided text  

    Result<T, E> can be also used to write tests 
        enables to use the question mark operator in the body, 
        which can be a convinient way to write test should fail 
        if any operation within them returns an Err variant 
        but cannot use the #[should_panic] annotation 

*/

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: usize) -> usize {
    a + 3 
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

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!") 
}

pub struct Guess {
    value: i32, 
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."); 
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."); 
        }

        Guess { value } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // test attribute  
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4); // macro (p1 == p2) 
        // test tests::exploration ... ok 
    }

    #[test]
    fn another() { 
        panic!("Make this test fail"); 
        // test tests::another ... FAILED
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8, 
            height: 7, 
        }; 
        let smaller = Rectangle {
            width: 5, 
            height: 1, 
        };

        assert!(larger.can_hold(&smaller)); 
        // test tests::larger_can_hold_smaller ... ok
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8, 
            height: 7, 
        }; 
        let smaller = Rectangle {
            width: 5, 
            height: 1, 
        };

        assert!(!smaller.can_hold(&larger)); 
        // test tests::smaller_cannot_hold_larger ... ok 
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(2); 
        assert_eq!(result, 4); 
        // assertion `left == right` failed
        // left: 5, right: 4
    }

    // adding custom failure messages: 
    #[test] 
    fn greeting_contains_name() {
        let result = greeting("Carol"); 
        assert!(result.contains("Carol"), 
            // add a custom failure message here 
            "Greeting did not contain name, value was `{result}`"
        );  
    }

    // checking for panics with should_panic 
    // to check that the code handles error conditions 
    #[test] 
    // expected param added to compare the message 
    #[should_panic(expected="less than or equal to 100")] 
    fn greater_than_100() {
        Guess::new(200); 
    }

    // using Result<T, E> in tests 
    #[test] 
    fn it_works() -> Result<(), String> {
        let result = add(2, 2); 
        
        if result == 4 {
            // when the test passes:  
            Ok(())
        } else {
            // when the test fails:  
            Err(String::from("two plus two does not equal four")) 
        }
    }

}
