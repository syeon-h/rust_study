/*
    Smart Pointers 
    smart pointers are a pointer that have additional metadata and capabilities 
        Ex. reference counting smart pointer enables the data to have 
            multiple owners by keeping track of the number of owners 
        while references only borrow data, smart pointers own the data 
        Ex. String and Vec<T> are smart pointers because they own some memory 
            and allow to manipulate it + metadata + extra capabilities 
        usually implemented using structs and smart pointers that implement: 
            Deref trait allows an instance to behave like a reference 
            Drop trait allows to handle when an instance goes out of scope 

    Box<T> allow to store data on the heap rather than the stack 
        the pointer to the heap data remains on the stack; do not have 
        performace overhead, other than storing their data on the heap 
        use boxes most often in: 
            1. type whose size cannot be known at compile time but want to use 
                in a context that requries an exact size 
            2. want to transfer ownership of a large amount of data but 
                ensure that data will not be copied 
            3. want to own a value that it is a type that implements 
                a particular trait rather than being of a specific type 

    recursive type have another value of the same type as part of itself 
        neting of values could theoretically continue infinitely, so Rust 
        cannot know how much space the value nedds; but boex have a known size 
    Ex. cons list is a data structure made up of nested pairs (linked list) 
        pseudocode representation: (1, (2, (3, Nil))) 
        each item contains two eleemnts: val of the current item and the next 
        the last item in the list contains only a value called Nil (not null) 

*/

// attemp to define a cons list: has infinite size 
//enum List {
//    Cons(i32, List), // holds another value directly 
//    Nil, 
// }

// definition of List that uses Box<T>: 
// boxes provie only the indirection and heap allocation 
enum List {
    Cons(i32, Box<List>), // size of i32 + to store the poitner data 
    Nil, // stores no values (size less than Cons) 
}

use crate::List::{Cons, Nil}; 

fn main() {
    // storing i32 val on the heap: 
    let b = Box::new(5); 
    println!("b = {b}"); 

    // let list = Cons(1, Cons(2, Cons(3, Nil))); 
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))); 
} // box will be deallocated (out of scope) 
