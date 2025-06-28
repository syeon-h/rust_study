/*
    Rc<T> is the reference counting smart pointer 
        enables multiple ownership explicitly by keeping track of 
        the number of references to a value to determine whether 
        or not the value is till in use (ex. graph data) 

        use when we want to allocate some data on the heap for
        multiple parts of our program to read and we cannot determine 
        at compile time which part will finish using the data last 

        only for use in single-thread scenarios 

    Ex. two lists b and c that both share ownership of a thrid list a: 
        b, instead of taking ownership of a, will clone the Rc<List>; 
        increase the number of references and let a and b share ownership 

    implementation of Rc::clone does not make a deep copy of all the data 
        only increments the reference count (does not take much time); 
        thus, can visually distinguish between the deep-copy kinds of 
        clones and the kinds of clones that increase the reference count 

    via immutable references, Rc<T> allows to share data between mulitple
        parts of the program for reading only 
*/

// definition of List that uses Rc<T>: 
enum List {
    Cons(i32, Rc<List>), 
    Nil, 
}

use crate::List::{Cons, Nil}; 
use std::rc::Rc; 

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); 
    // print the reference count 
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1
    let b = Cons(3, Rc::clone(&a)); 
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2 
    {
        let c = Cons(4, Rc::clone(&a)); 
        println!("count after creating c = {}", Rc::strong_count(&a)); // 3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2  
}
