/*
    unit tests test each unit of code in isolation from the rest 
        to quickly pinpoint where code is and is not working 
        put unit tests in the src directory in each file with the code  

    #[cfg(test)] to compile and run the test code only with cargo test 
        Rust do allow to test private functions too 
*/

pub fn add_two(a: usize) -> usize {
    internal_adder(a, 2)
}

// private function 
fn internal_adder(left: usize, right: usize) -> usize {
    left + right 
}

#[cfg(test)] // annotation to the tests module 
mod tests {
    use super::*;

    #[test]
    fn internal() {
        // testing a private function 
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
