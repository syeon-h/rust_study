/*
    statements are instrunctions that perform some action and DO NOT RETURN (;)
    expressions evaluate to a resultant value (no semi-colon at the end) 
*/

fn main() {
    // statement: cannot assigna a let statement to another var 
    // thus, x = y = 6 is not allowed in Rust (ex. let x = (let y = 6);)
    let y = 6; 

    // expression: can be part of statements  
    let y = {   // new scope block is an expression 
        let x = 3; 
        x + 1       // evaluates to 4 and gets bound to y 
    }; 

    another_function(5); 

    print_labeled_measurement(5, 'h'); 

    let x = five(); 
    println!("The value of x is: {x}"); 

    let x = plus_one(5); 
    println!("The value of x is: {x}"); 
}

// in fn signatures, must declare the type of each parameter 
fn another_function(x: i32) {
    println!("The value of x is: {x}"); 
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The parameter is: {value}{unit_label}"); 
}

// functions with return values: must declare their type 
fn five() -> i32 {
    5   // return value is synonymous with the fination expression 
}

fn plus_one(x: i32) -> i32 {
    x + 1 // placing ; at the end will change it to a statement 
}