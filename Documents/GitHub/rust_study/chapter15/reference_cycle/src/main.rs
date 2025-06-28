/*
    memory leak is to accidently create memory that is never cleaned up 
        Rust allows memory leaks by using Rc<T> and RefCell<T> where 
        items refer to each other in a cycle (reference cycle); 
        when having RefCell<T> values that contain Rc<T> values or 
        similar nested combinations of types with interior mutability, 
        must ensure that they do not create cycles (logic bug) 

    another solution is reorganizing the data structures so that some 
        refs express ownership and some ref do not; only the ownership 
        relationships affect whether or not a value can be dropped 

    weak references do not express an ownership relationship, and thier 
        count does not affect when an Rc<T> instance is cleaned up; 
        thus, will not cause a ref cycle because any cycle involving 
        some weak refs will be broken once the strong ref count is 0 

    Rc::downgrade returns a smart pointer of type Weak<T> and increases
        the weak_count by 1 (Rc::clone increases strong_count); 
        upgrade method returns an Option<Rc<T>>: 
            Some if the Rc<T> val has not been dropped yet 
            None if the Rc<T> val has been dropped 

    Ex. tree data struct whose items know about their children + parent: 
        want a Node to own its children, and share that ownership with 
        variables so each Node can be accessed directly; also want to 
        modify which nodes are children of another node, thus, have a 
        RefCell<T> in children around the Vec<Rc<Node>> 

        parent cannot contain an Rc<T> because that would create a ref 
        cycle with leaf.parent; a parent node should own its children 
        but a child should not own its parent, thus, use Weak<T> type 
        (parent need to be kept after their child gets dropped) 
*/

use crate::List::{Cons, Nil}; 
use std::cell::RefCell; 
use std::rc::{Rc, Weak}; 

// can modify the List val a Cons variant is pointing to: 
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>), 
    Nil, 
}

impl List {
    // to access the second item for a Cons variant 
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item), 
            Nil => None, 
        }
    }
}

#[derive(Debug)] 
// holds its own i32 val and refs to its children Node: 
struct Node {
    value: i32, 
    parent: RefCell<Weak<Node>>, 
    // define the Vec<T> items to be vals of type Rc<Node> 
    children: RefCell<Vec<Rc<Node>>>, 
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil)))); 

    println!("a initial rc count = {}", Rc::strong_count(&a)); // 1
    println!("a next item = {:?}", a.tail()); // Some(RefCell { value : Nil }) 

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a)))); 

    println!("a rc count after b creation = {}", Rc::strong_count(&a)); // 2 
    println!("b initial rc count = {}", Rc::strong_count(&b)); // 1 
    println!("b next item = {:?}", b.tail()); 
        // Some(RefCell { value: Cons(5, RefCell {value: Nil }) }) 

    // modify a so it points to b instead of Nil (ref cycle) 
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b); 
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // 2 
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); // 2 

    // println!("a next item = {:?}", a.tail()); 

    // instance with the val 3 and no children 
    let leaf = Rc::new(Node {
        value: 3, 
        parent: RefCell::new(Weak::new()), 
        children: RefCell::new(vec![]), 
    }); 

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); 
        // leaf parent = None 
    println!("leaf strong = {}, weak = {}", 
        Rc::strong_count(&leaf), Rc::weak_count(&leaf)); 
        // leaf strong = 1, weak = 0 
    
    // visualizing changes to strong_count and weak_count: 
    {   
        // instance with the val 5 and leaf as its child 
        let branch = Rc::new(Node {
            value: 5, 
            parent: RefCell::new(Weak::new()), 
            children: RefCell::new(vec![Rc::clone(&leaf)]), 
        }); 

        // modify leaf to give it a Weak<Node> ref to its parent 
        // Rc::downgrade fn to create a Weak<Node> ref to branch 
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); 

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); 

        println!("branch strong = {}, weak = {}", 
            Rc::strong_count(&branch), Rc::weak_count(&branch)); 
            // branch strong = 1, weak = 1 

        println!("leaf strong = {}, weak = {}", 
            Rc::strong_count(&leaf), Rc::weak_count(&leaf)); 
            // leaf strong = 2, weak = 0 
    } // branch out of scope (strong count decreases) 

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); 
        // leaf parent = None 

    println!("leaf strong = {}, weak = {}", 
        Rc::strong_count(&leaf), Rc::weak_count(&leaf)); 
        // leaf strong = 1, weak = 0 

}
