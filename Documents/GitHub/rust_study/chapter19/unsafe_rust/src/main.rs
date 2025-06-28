/*
    unsafe Rust is an second language hidden inside that does not enforce 
        the memory safety gurantees; gives an extre superpowers 

        to switch, use the unsafe keyword and then start a new block 

        unsafe superpowers are fives actions that can be taken:
            1. dereference a raw pointer 
            2. call an unsafe function or method 
            3. access or modify a mutable static variable 
            4. implement an unsafe trait 
            5. access fields of a union 

        unsafe does not turn off the borrow checker or diable any other 
        safety checks; only not checked by the compiler for memory safety 

        to isolate code as much as possible, it is best to enclose unsafe 
        code within a safe abstraction and provide a safe API 

    raw pointer can be immut or mut and are wirtten as *const T and *mut T
        immutable means that the pointer cannot be directly assigned to 
        after being dereferenced  (asterisk is part of the type name) 

        raw pointers are allowed to: 
        - ignore the borrowing rules by having both immut and mut pointer 
            or multiple mutable pointers to the same location 
        - are not guranteed to point to valid memory 
        - are allowed to be null 
        - do not implement any automatic cleanup 

        can create raw pointers in safe code but cannot dereference them 

    unsafe function and method has an extra unsafe keyword that indicates 
        the function has requirements need to uphold when being called

        do not need to mark the entire function as unsafe; wrapping them 
        in a safe function is a more common abstraction 

        Ex. split_at_mut takes one slice and makes it two by splitting 
        at the index given as an argument: attempt to implement using only 
        safe Rust will fail because borrow checker cannot understand that 
        we are borrowing different parts of the slice; it only knows that 
        we are borrowing from the same slice twice 

    extern facilitates the creation and use of Foreign Function Interface 
        Rust can interact with code written in another language 

        functions declared within extern blocks are always unsafe because 
        other languages do not enfore Rust's rules and guarantees 

    static variable (global variable) is similar to constant; subtle diff 
        is that values in a static var have a fixed address in memory; 
        using the value will always access the the same data 

        another diff is that static vars can be mutable, which is unsafe 

        names are in SCREAMING_SNAKE_CASE by convention 

        can only store references with the 'static lifetime, which means 
        the compiler can figure out it and are not required to annotate 

        to ensrue there are no data races, when possible, it is preferable 
        to use the concurrency techniques and thread-safe smart pointers 

    unsafe trait is when at least one of its methods have some invariant 
        that the compiler cannot verify; declared by adding the unsafe 
        keyword before the trait and the implementation of the trait 

        Ex. Sync and Send marker traits that contains a type that is not 
        Send or Sync, such as raw pointers 

    union is similar to a struct, but only one declared field is used 
        in a particualr instance at one time and are parimarily used to 
        interface with unions in C code 
        
        unsafe is used when accessing fields of a union because Rust 
        cannot guarantee the type of the data currently being stored 
*/

// declaring an extern function defined in C:  
// "C" defines application binary interface (ABI) 
extern "C" {
    fn abs(input: i32) -> i32; 
}

// declaring a mutable static variable: 
// specify mutability using the mut keyword 
static mut COUNTER: u32 = 0; 

fn add_to_count(inc: u32) {
    unsafe {
        // access and modify a static var: 
        COUNTER += inc; 
    }
}

// defining an unsafe trait: 
unsafe trait Foo {
    // methods  
}

// implementing an unsafe trait 
unsafe impl Foo for i32 {
    // method implementation 
}

fn main() {
    let mut num = 5; 

    // creating raw pointers from reference (guaranteed) 
    let r1 = &num as *const i32; 
    let r2 = &mut num as *mut i32; 

    // raw pointer whose validity cannot be guranteed 
    let address = 0x12345usize; 
    let r = address as *const i32; 

    // dereferencing raw pointers within unsafe 
    unsafe {
        println!("r1 is: {}", *r1); 
        println!("r2 is: {}", *r2); 
    }

    // using the safe split_at_mut function 
    let mut v = vec![1, 2, 3, 4, 5, 6]; 
    let r = &mut v[..]; 
    let (a, b) = r.split_at_mut(3); 

    assert_eq!(a, &mut [1, 2, 3]); 
    assert_eq!(b, &mut [4, 5, 6]); 

    // extern function calling example: 
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3)); 
    }

    // mutable static variable usage example: 
    add_to_count(3); 

    unsafe {
        println!("COUNTER: {COUNTER}"); 
    }
}

use std::slice; 

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len(); 
    // as_mut_ptr access the raw pointer of a slice (*mut i32) 
    let ptr = values.as_mut_ptr(); 

    assert!(mid <= len); 

    unsafe {
        (
            // takes a raw pointer and a length, and creates a slice 
            // unsafe becasue it must trust that the raw pointer is valid 
            slice::from_raw_parts_mut(ptr, mid), 
            slice::from_raw_parts_mut(ptr.add(mid), len - mid), 
        )
    }
}