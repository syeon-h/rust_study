use std::env; 
use std::process; 
use grep_project::Config; 

/*
    env::args returns an iterator; now we are passing ownership of 
        the iterator returned from env::args to Config::build directly 
*/
fn main() {
    // let config = Config::build(&args).unwrap_or_else(|err| {
    // passing the return val of env::args to Config::build 
    let config = Config::build(env::args()).unwrap_or_else(|err| {

        eprint!("Problem parsing arguments: {err}"); 
        process::exit(1); 
    }); 

    // println!("Searching: {}", config.query); 
    // println!("In the file: {}", config.file_path); 

    if let Err(e) = grep_project::run(config) {
        eprintln!("Application error: {e}"); 
        process::exit(1); 
    }
}