/*
    integrationt tests are entirely external to your library 
        use it in the same way any other code would, can only call 
        functions that are part of your library's public API 
    
    to test whether many parts of your library work toegether correctly 
        units of code that work correctly on their own could have 
        problems when integrated 

    create a tests directory at the top level of our project directory 
        each file in the tests directory is a separate crate 
        do not need to annotate with #[cfg(test)] because Cargo treats 
        the tests directory specially and compiles only with cargo run 

    three sections of output include: unit, integration, and doc tests 
        if any test in a section fails, the following one will not be run 
        can run a particular integrationt test function by specifying 

    submodules in integration tests: more files in the tests 
        Ex. group the functions by the functionality they are testing 
        files in the tests do not share the same behaviour as in src 
        thus, must create mod.rs file under a new directory 
        Ex. tests/common/mod.rs and use with mod common; 

    integration tests for binary creates: only contains a src/main.rs,  
        cannot create integration tests and bring functions into scope 
        only library creates expose functions that other crates can use; 
        binary crates are meant to be run on their own 
*/

// bring our library into test crate's scope 
use test_organization::add_two; 

// do not need to annotate #[cfg(test)] 
#[test]
fn it_adds_two() {
    let result = add_two(2); 
    assert_eq!(result, 4); 
}