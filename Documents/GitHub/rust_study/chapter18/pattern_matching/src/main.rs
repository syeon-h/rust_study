/*
    match arms: use patterns in the arms of match expressions 
        match VALUE {
            PATTERN => EXPRESSION, 
            PATTERN => EXPRESSION, 
        }

        one requirement is that they need to be exhaustive in the sense 
        that all possibilities for the value must be accounted for; one
        way to ensure is to have a catchcall pattern for the last arm 
        (ex. _ will match anything, but it never binds to a variable) 

    conditional if let: shorter way to write the equivalent of a match 
        that only matches one case; can have a corresponding else 

        also possible to mix and match if let, else if, and else if let; 
        Rust does not require that the conditions relate to each other 

        can introduce shadowed var in the same way that match arms can 
        (ex. if let Ok(age) = age introduce a new shadowed age var that 
        contains the value inside the Ok variant) 

        downside is that the compiler does not check for exhaustiveness, 
        whereas with match expressions it does (possible logic bug) 

    while let conditional loops: allow a while loop to run for as long as 
        a pattern continues to match (ex. to pop every element off) 

    for loops: value that directly follows the keyword for is a pattern 
        (ex. in for x in y, the x is the pattern) 

    let statements: looks like let PATTERN = EXPRESSION; 
        the variable name is just a particularly simple form of a pattern 

        Ex. in the let x = 5;, x is a pattern that means "bind what 
        matches here to the variable x" 

        let can destructure a tuple, which is the pattern matching aspect

    function parameters: can also be patterns 
        fn foo(x: i32) { --snip-- }, the x part is a pattern 

        could match a tuple in a function's args to the pattern 
        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({x}, {y})"); 
        }
        fn main() {
            let point = (3, 5); 
            print_coordiates(&point); 
        }
        the values &(3, 5) match the pattern &(x, y) 

        can also use patterns in closure parameter lists in the same way 
        as in function parameter lists (closures are similar to funcs) 
    
    patterns come in two forms: refutable and irrefutable 
        irrefutable patterns will match for any possible value passed 
        (ex. let x = 5;, x matches anything, thus, cannot fail) 

        refutable patterns can fail to match for some possible value 
        (ex. if let Some(x) = a_val, if the a_val is None, the Some(x) 
        pattern will not match)

        function parameters, let statements, and for loops can only accept 
        irrefutable, because cannot do anything when values do not match 

        if let and while let expressions accept refutable and irrefutable, 
        but the compiler warns against irrefutable 

        if a refutable pattern is given where an irrefutable is needed, 
        can fix it by changing the code: use if let instead of let 
        

*/

fn main() {
    // conditional if let example: 
    let favourite_colour: Option<&str> = None; 
    let is_tuesday = false; 
    let age: Result<u8, _> = "34".parse(); // parse as a number 

    if let Some(colour) = favourite_colour {
        // if user specifies a favourite colour 
        println!("Using your favourtie colour: {colour}"); 
    } else if is_tuesday {
        // if is_tusday is true 
        println!("Tuesday is green day!"); 
    } else if let Ok(age) = age {
        // if age is specified 
        if age > 30 {
            println!("Using purple"); 
        } else {
            println!("Using orange"); 
        }
    } else {
        // if none of these apply 
        println!("Using blue"); 
    }

    // while let conditional loop example: 
    let mut stack = Vec::new(); 

    stack.push(1); 
    stack.push(2); 
    stack.push(3); 

    while let Some(top) = stack.pop() {
        println!("{top}"); 
    }

    // for loop example: 
    let v = vec!['a', 'b', 'c']; 

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}"); 
    }

    // let statement example: 
    let (x, y, z) = (1, 2, 3);  

    // function parameter example: 
    fn foo(x: i32) {
        // code goes here 
    }
}
