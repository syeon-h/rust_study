use std::error::Error; 
use std::fs; 
use std::env; 

// group configuration values 
pub struct Config {
    pub query: String, 
    pub file_path: String, 
    // option to switch between case-sensitive
    // and case-insensitive search 
    pub ignore_case: bool, 
}

// constructor for Config 
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // returns a Result with a Config instance if success 
        // and string literal in the error case 
        if args.len() < 3 {
            return Err("not enough arguments"); 
        }

        let query = args[1].clone(); 
        let file_path = args[2].clone();  

        // var function to check to see if any value has been set 
        // ex. $ IGNORE_CASE=1 cargo run -- to poem.txt 
        let ignore_case = env::var("IGNORE_CASE").is_ok(); 

        Ok(Config { query, file_path, ignore_case, })
    }
}

// extracting a run function containing logic 
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read the contents of the file_path 
    let contents = fs::read_to_string(config.file_path)?; 

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    // use the search function 
    for line in results {
        println!("{line}"); 
    }

    Ok(())
}

/*
    search function will do the following: 
        1. iterate through each line of the contents 
        2. check whether the line contains the query 
        3. if it does, add it to the list of return values 
        4. if it does not, do nothing 
        5. return the list of results that match 
*/

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // recall: lifetime params specify which arg lifetime is 
    // connected to the lifetime of the return value 

    let mut results = Vec::new(); 
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim_end()); 
        }
    }

    results 
}

/*
    Working with Environment Variables 
    case-sensitive searching that can be turned on via an environment variable 
        command line option that users enter 
        allow to set the environment variable once and last for that session 
*/

pub fn search_case_insensitive<'a>(
    query: &str, 
    contents: &'a str, 
) -> Vec<&'a str> {
    let query = query.to_lowercase(); 
    let mut results = Vec::new(); 

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.trim_end()); 
        }
    }

    results 
}

/* 
    Developing the library's functionality with TDD 
    test-driven development (TDD) process: 
        1. wirte a test that fails and run it 
        2. write or modify just enough code to make the new test pass 
        3. refactor the code just added or changed and make continue to pass 
        4. repeat from step #1 
*/

#[cfg(test)] 
mod tests {
    use super::*; 

    #[test] 
    fn one_result() {
        // searches for "duct" 
        let query = "duct"; 
        let contents = "\
Rust: 
safe, fast, productive. 
Pick three."; 

        assert_eq!(vec!["safe, fast, productive."], search(query, contents)); 
    }

    #[test] 
    fn case_sensitive() {
        let query = "duct"; 
        let contents = "\
Rust: 
safe, fast, productive. 
Pick three.
Duct tape."; 
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents)); 
    }

    #[test] 
    fn case_insensitive() {
        let query = "rUsT"; 
        let contents = "\
Rust:
safe, fast, productive. 
Pick three.
Trust me."; 
        
        assert_eq!(
            vec!["Rust:", "Trust me."], 
            search_case_insensitive(query, contents)
        ); 
    }
}

