use rand; 

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2)); 
    }
}

/*
    can run tests for one particular crate in a workspace from
        the top-level directory by using -p flag 
        Ex. cargo test -p add_one 
*/