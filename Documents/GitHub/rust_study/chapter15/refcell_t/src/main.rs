/*
    combining Rc<T> and RefCell<T> is a common way: 
        recall: Rc<T> lets you have multiple owners of some data 
        if you have an Rc<T> that holds a RefCell<T>, can get 
        a value that can have multiple owners AND that can be mutated 

    Ex. add in RefCll<T> to the cons list to gain the ability to change 
        the values in the lists 
        by using RefCell<T>, have an outwardly immutable List value;  
        but can use the methods on RefCell<T> that provide access to its 
        interior mutability so can modify our data when needed 
        
*/

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>), 
    Nil, 
}

use crate::List::{Cons, Nil};
use std::cell::RefCell; 
use std::rc::Rc;  

fn main() {
    // instance of Rc<RefCell<i32>> 
    let value = Rc::new(RefCell::new(5)); 

    // List a with a Cons variant that holds value 
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil))); 

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)); 
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a)); 

    // deref the Rc<T> to the inner RefCell<T> value 
    *value.borrow_mut() += 10; // returns a RefMut<T> 

    println!("a after = {a:?}"); 
    println!("b after = {b:?}"); 
    println!("c after = {c:?}"); 

}