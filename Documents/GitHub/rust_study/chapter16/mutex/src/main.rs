/* 
    shared memory concurrency is like multiple ownership: multiple threads 
        can access the same memory location at the same time; but can add 
        complexity because these different owners need managing 

    mutex allows only one thread to access some data at any given time; 
        to access the data, a thread must first signal that it wants 
        access by asking to acquire the mutex's lock
        
        lock is a data structure that keeps track the execlusive access; 
        thus, mutex is GUARDING the data it holds via the locking system 

        mutexes have two important rules: 
            1. must attempt to acquire the lock before using the data 
            2. when done with the data, must unlock the data 

    Mutex<T> is a smart pointer, created using the associated function new 
        use the lock method to acquire the lock: will block the current 
        thread so it cannot do any work until it gets the turn 

        call would fail if another thread holding the lock panicked; in 
        that case, no one would ever able to get the lock, so unwrap() 
        and have this thread panic if they are in such situation 

        call to lock returns a smart pointer called MutexGuard, wrapped 
        in a LockResult that can be handled with the call to unwrap; 
        implements Deref to point at the inner data and Drop that releases 
        the lock automatically when a MutexGuard goes out of scope 

    Ex. to share a value between multiple threads using Mutex<T>: 
        first attempt fails as the counter val was moved in the previous 
        iteration of the loop; cannot move the ownership of counter into 
        multiple threads 

        second attempt using Rc<T> also fails as Rc<Mutex<i32>> cannot be 
        sent between threads safely as the trait Send is not implemented 
        for Rc<Mutex<i32>>; does not use any concurrency primitives to 
        make sure that changes to the count cannot be interrupt by another

    Arc<T> is like Rc<T> that is safe to use in concurrent situations 
        A stands for ATOMIC: an atomically reference counted type; atomics
        work like primitive types but are safe to share across 

        not default type as thread safety comes with a performance penalty 
        that you only want to pay when it is needed (ex. single thread) 

        Arc<T> and Rc<T> have the same API 

    Recall: counter is immut but could get a mut ref to the val inside it 
        Mutex<T> provides interior mutability as the Cell family does; 
        Rust cannot protect all kinds of logic errors with Mutex<T>, thus, 
        comes with the risk of creating deadlock  

    Send marker trait indicates that ownership of the vals of the type 
        implementing Send can be transferred between threads; 

        Rc<T> cannot be Send becuase if you cloned an Rc<T> val and 
        tried to transfer ownership of the clone, both threads might 
        update the ref count at the same time, thus, single-threaded only 

        almost all primitive types are Send, aside from raw pointers 

    Sync market trait indicates that it is safe for the type implementing 
        Sync to be referenced from multiple threads; any type T is Sync 
        if &T (immut ref) is Send, meaning the ref can be sent safely to 

        primitive types are Sync, and types composed entirely of type 
        that are Sync are also Sync 

        Rc<T> is also not Sync for the same reasons that it is not Send; 
        RefCell<T> and the family of related Cell<T> types are not Sync; 
        (as borrow checking at runtime is not thread-safe) 

    manially implementing these traits involves implementing unsfae code  
*/

use std::sync::{Mutex, Arc}; 
use std::thread; 

fn main() {
    // the API of Mutex<T> in a single-threaded context: 
    // type of m is Mutex<i32> not i32 
    let m = Mutex::new(5); 

    {
        let mut num = m.lock().unwrap(); 
        *num = 6; 
    } // lock is released automatically 

    println!("m = {m:?}"); 

    // multiple ownership with multiple threads: 
    // attempt#1: let counter = Mutex::new(0); 
    // attempt#2: let counter = Rc::new(Mutex::new(0)); 
    let counter = Arc::new(Mutex::new(0)); 
    let mut handles = vec![]; 

    // create 10 threads: 
    for _ in 0..10 {
        // attempt#2: let counter = Rc::clone(&counter); 
        let counter = Arc::clone(&counter); 
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); 

            *num += 1; 
        }); 
        handles.push(handle); 
    }

    for handle in handles {
        // join to make sure all the threads finish 
        handle.join().unwrap(); 
    }

    println!("Result: {}", *counter.lock().unwrap()); 
        // Result: 10 
}
