/*
    ownsership is a set of rules that govern how a Rust manages memory: 
        1. each value in Rust has an owner 
        2. there can only be one owner at a time 
        3. when the owner goes out of scope, the value will be dropped (free)
*/

fn main() { 
    {   // s is not valid (not yet declared) 
        let s = "hello"; // s is valid from this point forward (immutable) 
        // s is a string literal: value is hardcoded and can be stored on the stack 
    }   // this scope is now over; s is no longer valid 

    // String type manages data allocated on the heap (unknown at compile time)
    let mut  s = String::from("hello"); 
    s.push_str(", world!"); // appends a literal to a string 
    println!("{s}"); 

    /*
        With the String to support a mutability: 
            1. the memory must be requested from the memory allocator at runtime 
            2. need a way of returning this memory to the allocator (drop) 

        First part is done by calling String::from, its implementation 
        Sceond part is done automatically once the variable goes out of scope 
    */

    // move: multiple variables interact with the same data in different ways 
    // simple (stack-only) data will bind the value and then make a copy 
    // ex. integer, Boolean, floating point, character, tuples including these types 
    let x = 5; 
    let y = x; // x = 5 and y = 5 

    // complex data types do not copy the data but moves the data 
    // s1 was 'moved' into s2, thus, s1 is no longer valid 
    let s1 = String::from("hello"); 
    let s2 = s1; // s1 is not valid from this point 
    // println!("{s1}"); will occur an error 
    
    // clone: to deeply copy the heap data of the String (expensive) 
    let s3 = String::from("hello"); 
    let s4 = s3.clone(); 
    println!("s3 = {s3}, s4 = {s4}"); 

    // ownership and functions: passing a value to a function is similar 
    let s = String::from("hello"); // s comes into scope 
    takes_ownership(s); // s's value 'moves' into fn and no longer valid  

    let x = 5;  // x comes into scope 
    makes_copy(x); // x would move into fn but i32 is 'copied'; still valid 

    // returing values: can also transfer ownership 
    let s5 = gives_ownership(); // gives_ownership moves its return value into s5 
    
    let s6 = String::from("hello"); 
    let s7 = takes_and_gives_back(s6); // s6 is moved into fn, which also 
                                                        // moves its return value into s7   
} 

fn takes_ownership(some_str: String) {
    println!("{some_str}"); 
} // some_str goes out of scope and drop is called (memory is freed) 

fn makes_copy(some_int: i32) {
    println!("{some_int}"); 
} // some_int goes out of scope but nothing special happens 

fn gives_ownership() -> String {
    let some_str = String::from("yours"); 
    some_str // some_str is returned and moves out to the calling fn 
}

// takes a String and returns one 
fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moves out to the calling fn 
}