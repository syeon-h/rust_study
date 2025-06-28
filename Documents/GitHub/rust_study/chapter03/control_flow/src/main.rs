fn main() {
    // if expressions: to branch the codes depending on conditions 
    let number = 6; 

    if number % 4 == 0 {
    // if number { will NOT work as the condition must be a bool 
        println!("number is divisible by 4"); 
    } else if number % 3 == 0 {
        println!("number is divisible by 3"); 
    } else if number % 2 == 0 {
        println!("number is divisible by 2")
    } else {
        println!("number is not divisible by 4, 3, or 2"); 
    }

    // using if in a let statement: to assign the outcome 
    let condition = true; 
    let number = if condition { 5 } else { 6 }; 
    // let number = if condition { 5 } else { "six" }; will NOT work 
    // the values from each arm must be the same type 
    println!("The value of number is: {number}"); 

    /*
        Rust has three kinds of loops: 
            1. loop tells to execute a block of code over and over again forever 
            2. while loop to run code while a condition holds true 
            3. for loop executes some code for each item in a collection 
    */

    // loop: can add the value you want returned after the break 
    let mut counter = 0; 
    
    let result = loop {
        counter += 1; 

        if counter == 10 {
            break counter * 2; 
        }
    }; 
    
    println!("The result is {result}"); 

    // while: eliminates a lot of nesting that would be necessary 
    let mut number = 3; 

    while number != 0 {
        println!("{number}!"); 
        number -= 1; 
    }

    // for: looping thorugh each element of a collection 
    let a = [10, 20, 30, 40, 50]; 

    for element in a {
        println!("The value is: {element}"); 
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
}
