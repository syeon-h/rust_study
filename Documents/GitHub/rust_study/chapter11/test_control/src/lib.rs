/*
    specify command line options to change the default behaviour 
        some go to cargo test, and some go to the resultant test binary 
        to separate these two types, list the arguments 
        that go to cargo test followed by the separator -- and 
        then the one that go to the test binary 
    
    running tests consecutively: -- --test-threads=1 
        to run the tests one at a time (not parallel) 
    
    showing function output: -- --show-output 
        to see printed values for passing tests 
    
    running single tests: ex. cargo test one_hundred 
        pass the name of any test function to run only that 
    
    filtering to run multiple tests: ex. cargo test add  
        specify part of a test name, and any test 
        whose name matches that value will be run 

    ignoring some tests: #[ignore] after #[test] 
        -- --ignored to run only the ignored test 
        -- --include-ignored to run all tests (ignored or not) 

*/

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: usize) -> usize {
    a + 2 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // cargo test and will run the two tests: 
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

    // cargo test one_hundred will only run:  
    #[test] 
    fn one_hundred() {
        let result = add_two(100); 
        assert_eq!(result, 102);  
    }

    // annotate the time-consuming tests 
    #[test]
    #[ignore] 
    fn expensive_test() {
        // code that takes an hour to run 
    }
}

