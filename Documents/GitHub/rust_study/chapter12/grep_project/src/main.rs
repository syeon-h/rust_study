/*
    Building a Command Line Program Project 

    issue #1: main function is performing two tasks (too large)
    issue #2: config and logic variables are mixed 
    issue #3: error message is not giving enough information 
    issue #4: lack of error-handling code 
    
    Refactoring process 
    1. splitting the separate concerns of binary program 
        - our main perfroms two tasks: it parses arguments and reads files 
        - split into a main.rs and a lib.rs and move the logic to lib.rs 
        - responsbilities that remain in the main function are: 
            calling the command line parsing logic, 
            setting up any other configuration, 
            calling a run function in lib.rs 
            handling the error if run returns an error 
        - group config values 
    2. fixing the error handling 
        - recall: program will panic if the args contains fewer than three 
        - improve the error message using panic! macro
            Ex. panic!("not enough arguments"); 
        - return a result instead of calling panic! macro 
        - call config::build and handling errors  
        - unwrap_or_else allows to define some custom error handling 
            pass the inner value of the Err 
        - process::exit will stop the program and return the number 
            nonzero exit status is a convention to signal to the process 
    3. extracting logic from main (run function) 
        - can improve the error handling in the logic function (Result) 
        - Box<dyn Error> trait object means the function will return 
            a type that implements the Error trait (without specifying) 
        - ? operator will return the error value for the caller to handle 
        - Ok(()) syntax is the idiomatic way to indicate that run is called 
            for its side effects only (does not return a value) 
    4. splitting code into a lib crate 
        - move all the code that is not in the main function: 
            run function definition, relevant use statements, 
            Config definition, Config::build function definition 

*/

// read the values of command line args 
use std::env; 
use std::process; 

// bring the code moved to lib.rs into the scope 
use grep_project::Config; 

fn main() {
    // collect the args into a vector 
    // first value is the name of our binary 
    let args: Vec<String> = env::args().collect(); 

    let config = Config::build(&args).unwrap_or_else(|err| {
        // error message to standard error 
        eprint!("Problem parsing arguments: {err}"); 
        process::exit(1); // stop the program 
    }); 

    println!("Searching: {}", config.query); 
    println!("In the file: {}", config.file_path); 

    // handling errors returned from run 
    if let Err(e) = grep_project::run(config) {
        eprintln!("Application error: {e}"); 
        process::exit(1); 
    }
}

/*
    Writing Error Messages to Standard Error 
    two kinds of output: 
        standard output (stdout) for general info 
        standard error (stderr) for error messages 

    print! macro is only capable of printing stdout 
        > syntax tells to write the contents of stdout to file 
        Ex. cargo run > output.txt will give 
            Problem parsing arguments: not enough arguments 
        more useful for error messages to be printed to stderr 
        so only data from a successful run ends up in the file 
    
    printing errors to stderr using eprintln! macro 
        now see the error onscreen and output.txt contains nothing  
*/