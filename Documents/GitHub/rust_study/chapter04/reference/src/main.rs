/*
    reference(&) is like a pointer in that its an address we can follow to access the data 
        unlike a pointer, guaranteed to point to a valid value for the life of that reference 
*/

fn main() {
    let s1 = String::from("hello"); 
    // &s1 allow you to refer to some value without taking ownership of it 
    let len = calculate_length(&s1); 
    println!("The length of '{s1}' is {len}."); 

    // change(&s1) does not work and occur an error 
    
    // mutable reference: if you have a mut reference to a value, 
    // you can have no other references to that value 
    let mut s2 = String::from("hello"); 
    change_mut(&mut s2); 

    // cannot borrow s3 as mutable more than once at a time 
    // to prevent data races 
    let mut s3 = String::from("hello"); 
    let r1 = &s3;   // no problem 
    let r2 = &s3;   // no problem 
    let r3 = &mut s3;   // PROBLEM 
    println!("{r1}, {r2}, and {r3}"); // will not work 

    /*
        dangling pointer is a pointer that references a location in memory that 
            may have been given to someone else (freeing some memory while preserving a poitner) 
        
        In Rust, the compiler guarantees that references will never be dangling references
            if you have a ref, the compiler will ensure that the data will not go out of scope 
    */
    // this reference would be pointing to an invalid String (error) 
    let reference_to_nothing = dangle(); 
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String 
    s.len() 
} // s goes out of scope, but because it does not have ownsership (not dropped) 

// attempting to modify a borrowed value 
fn change(some_str: &String) {
    some_str.push_str(", world"); 
}

// mutable reference: 
fn change_mut(some_str: &mut String) {
    some_str.push_str(", world"); 
}

// error missing lifetime specifier 
// this fn's return type contains a borrowed value 
// but there is no value of it to be borrowed from 
fn dangle() -> &String { // dangle returns a ref to a String 
    let s = String::from("hello"); // s is a new String 
    &s 
    // the solution is to return the String directly as s    
} // s goes out of scope, and is dropped; memory goes away 