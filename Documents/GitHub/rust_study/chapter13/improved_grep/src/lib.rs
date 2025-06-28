use std::error::Error; 
use std::fs; 
use std::env; 

pub struct Config {
    pub query: String, 
    pub file_path: String, 
    pub ignore_case: bool, 
}

/*
    improve#1: removing a clone using an iterator 
        needed clone because we have a slice with String elements in 
        the param args, but the build function does not own args;  
        thus, can change the build to take ownership of an iterator 

    env::args returns itertor of type std::env::Args, and that type 
        implements the Iterator trait and returns String vals 
        Ex. args has a generic type with the trait bounds 
            impl Iterator<Item = Sring> 
*/

impl Config {
    pub fn build(
        // update the sig to expect an iterator 
        mut args: impl Iterator<Item = String>, 
    ) -> Result<Config, &'static str> {
        args.next(); // first val is the name of the program 

        // use iterator trait methods instead of indexing 
        let query = match args.next() {
            Some(arg) => arg, 
            None => return Err("Didn't get a query string"), 
        }; 

        let file_path = match args.next() {
            Some(arg) => arg, 
            None => return Err("Didn't get a file path"), 
        }

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
    improve#2: making clearer with iterator adapters 
        use iterator adapter methods: can avoid having a mutable 
        imtermediate results vector (thus, no concurrent access) 
*/

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // recall: search is to return all lines that contains the query 
    contents
        .lines()
        // filter to keep only the lines that line.contains(query) 
        .filter(|line| line.contains(query))
        // then collect the matching line into another vec 
        .collect() 
}

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

