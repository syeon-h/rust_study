/*
    Rust groups errors into two major catergories: 
        1. recoverable mostly want to report the problem and retry 
            Ex. file not found error 
        2. unrecoverable is symptom of bugs and immediately stop the program 
            Ex. trying to access a location beyound the end of an array 

    panic! macro is caused when there's nothing you can do about it 
        print a failure message, unwind, clean up the stack, and quit 
        can explicitly call it: panic!("crash and burn"); 
    
    backtrace figure out the part of the code that is causing the problem 
        Ex. index out of bounds will stop excution and refuse to continue 
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace 

    recoverable errors are not serious enough to requrie to stop entirely 
        Ex. if you try to open a file and that operation fails 
            because the file does not exist, might want to create the file 
            instead of terminating the process 
        recall: Result enum is defined as having two variants: 
            enum Result<T, E> {
                Ok(T),  // T represents the type that will be returned 
                Err(E), // E represents the type of the error 
            } 

    propagating return the error to the calling code so that it can decide what to do 
        gives more control to the calling code, where there might be more 
        information or logic that dectates how the error should be handled 
        Ex. if the file does not exist or cannot be read, 
            return those errors to the code that called the fn 

    ? operator go thorugh the from function to convert values into another type 
        the error type received is converted into the error type 
        defined in the return type of the current function 
        Ex. could change the fn to return a custom error type named OurError 
            then the ? calls in the body will call from and convert it 
        eliminates a lot of boilerplate and makes the implementation simpler 

    ? operator can only be used in fns whose return type is compatible 
        Ex. attmpting to use in the main fn that returns () will not work 
            let greeting_file = File::open("hello.txt")?; 
            ? operator follows the Result value returned by File::open, 
            but this main fn has the return type of (), not Result 
        to fix the error: 
            1. to change the return type of the function 
                Ex. fn main() -> Result<(), Box<dyn Error>> 
                Box<dyn Error> eman "any kind of error" 
            2. to use a match or one of the Result<T, E> methods 
    
    returning Result is a good default chocie when defining a fn that might fail 
        however, in situations such as exmplaes, prototype code, and tests, 
        it's more appropriate to write code that panics 
*/

use std::fs::{self, File}; 
use std::io::{self, Read, ErrorKind}; 

fn main() {
    // explicitly call panic! macro: 
    // panic!("crash and burn"); 

    // return type of File::open is a Result<T, E> 
    // if succeeds: instance of Ok that contains a file handle 
    // if fails: instance of Err that contains more info 
    let greeting_file_result = File::open("hello.txt"); 

    // to handle the Result variants: 
    let greeting_file = match greeting_file_result {
        Ok(file) => file, 
        // returns inside the Err is io::Error 
        Err(error) => match error.kind() {
            // if the file trying to open does not exist yet: 
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc, 
                Err(e) => panic!("Problem creating the file: {e:?}"), 
            }, 
            // handle any other error type: 
            other_error => {
                panic!("Problem opening the file: {error:?}"); 
            }
        }, 
    }; 

    // shorcut for panic on error: unwrap  
    // if Ok, unwrap will return the value inside the Ok 
    // if Err, unwrap will call the panic! macro 
    let greeting_file = File::open("hello.txt").unwrap(); 

    // shorcut for panic on error: expect 
    let greeting_file = File::open("hello.txt")
        // lets us also choose the panic! error message 
        .expect("hello.txt should be included in this project"); 
}

// propagaing errors: return to the calling code 
// if Ok, return Ok value that holds a String(username) 
// if Err, return io::Error that contains more info 
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt"); 

    let mut username_file = match username_file_result {
        Ok(file) => file, 
        Err(e) => return Err(e), 
    }; 

    let mut username = String::new(); 

    // to read the contents of the file into username 
    // read_to_string also returns a Result 
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username), 
        Err(e) => Err(e), // up to the calling code 
    }     

}

// shortcut for propagating: ? operator 
// if Ok, the value inside will get returned 
// if Err, Err will be returned from the whole fn 
fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; 
    let mut username = String::new(); 
    username_file.read_to_string(&mut username)?; 
    Ok(username)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new(); 
    File::open("hello.txt")?.read_to_string(&mut username)?; 
    Ok(username) 
}

// fs::reand_to_string creates a new String, reads the contents, 
// put the contents into that String and returns it 
fn read_username_from_file_more_shorter() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt") 
}

// using ? operator on an Option<T> value 
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last() 
}