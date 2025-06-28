/*
    Deref trait allows to customize the behaviour of the deref operator (*) 
        smart pointer can be treated like a regular ref, 
        without, the compiler can only deref & references;  
        thus, gives the ability to take a value of any type that implements 

    Ex. define a smart pointer MyBox<T> similar to the Box<T> type: 
        Box<T> is ultimately defined as a tuple struct with one element 
        attempt to use MyBox::new(x) will fail as it cannot be dereferenced 

    deref coercion converts a ref to a type that implements the Deref trait 
        into a ref to another type (ex. convert &String to &str); 
        happens automatically when a ref is passed as an arg to a function 
        or method that does not match the param type in the definition; 
        sequence of calls to the deref method converts the type (into param) 
        Ex. Rust calls deref again to turn &String into &str: 
            standard lib provides an implementation of Deref on String
    
    DerefMut trait can be used to override the * operator on mutable refs 
        Rust does deref coercion in three cases: 
            1. from &T to &U when T: Deref<Target=U> 
            2. from &mut T to &mut U when T: DerefMut<Target=U> 
            3. from &mut T to &U when T: Deref<Target=U> 
        Rust coerce a mut ref to an immutable one on the third case 
        but the reverse is NOT possible: immutable ref will never coerce 
        to mut ref because of the borrowing rules (mut ref must be the only) 
        Ex. converting mut ref to immutable ref will never break the rules 
        Ex. converting immutable ref to a mut ref would require that the 
            initial immutable ref is the only immutable ref to that data, 
            but the borrowing rules do not guarantee that 
    
    Drop trait allows to customize what happens when a val goes out of scope 
        Ex. when a Box<T> is dropped it will deallocate the space on the heap 
        requires to implement drop method that takes a mutable ref to self 
        Drop trait is included in the prelude; drop will be called in the 
        reverse order of their creation (ex. d was dropped before c) 

    std::mem::drop function force to be dropped before the end of its scope 
        Ex. when using smart pointers that manage locks: want to force the 
            drop that releases the lock so that other code can acquire it 
        destructor is the general term for a fn that cleans up an instance 
        
*/

use std::ops::Deref; 

struct MyBox<T>(T); 

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x) 
    }
    // attempt to use MyBox::new(x) will fail at this point 
    // as it cannot be dereferenced (not implemented yet)   
}

impl<T> Deref for MyBox<T> {
    // defines an associated type for the trait: 
    // (different way of declaring a generic param) 
    type Target = T; 

    fn deref(&self) -> &Self::Target {
        &self.0 // returns a ref to the first val 
    }
}

// fn that has the param of type &str: 
fn hello(name: &str) {
    println!("Hello, {name}!"); 
}

struct CustomSmartPointer {
    data: String, 
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // logic that runs when an instance goes out of scope 
        println!("Dropping CustomSmartPointer with data `{}`!", self.data); 
    }
}

fn main() {
    // using deref operator to follow a ref: 
    let x = 5;   // holds an i32 val 
    let y = &x; // ref to x 

    assert_eq!(5, x);   
    assert_eq!(5, *y); //ref to val in y = *(&x) 

    // using deref operator on a Box<T>: 
    // pointing to a copied val of x 
    let y = Box::new(x); 

    assert_eq!(5, x); 
    assert_eq!(5, *y); // *(y.deref()) 

    // calling hello with a ref to a MyBox<String> val: 
    let m = MyBox::new(String::from("Rust")); 
    hello(&m); // identical to: hello(&(*m)[..]);  

    let c = CustomSmartPointer {
        data: String::from("my stuff"), 
    }; 
    let d = CustomSmartPointer{
        data: String::from("other stuff"), 
    }; 
    println!("CustomSmartPointers created."); 
    
    // c.drop(); not works (not allowed to explicitly call) 
    // as Rust would still automatically call drop at the end 

    drop(c); // calling std::mem::drop to explicitly drop a val 
    // Dropping CustomSmartPointer with data `my stuff`! 
    println!("CustomSmartPointer dropped before the end of main."); 
} // Rust will call the drop method here 
