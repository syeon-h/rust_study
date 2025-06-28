/*
    executed program's code is run in a process, and within a program, 
        there can be indepedent parts (thread) that run simultaneously 

        splitting the computation can improve performance, but it also 
        adds complexity; there is no inherent guarantee about the order; 
        - race conditions: threads are accessing in an inconsistent order 
        - deadlocks: two threads are waiting forever for each other 
        - bugs: only in certain situations and are hard to reproduce  
    
    thread::spawn fn takeing a closure (code) creates a new thread 
        all spawned threads are shut down when the main thread completes 
        (whether or not they have finished running); can be fixed by 
        saving the return val in a variable and calling the join method 

        return type of thread::spawn is JoinHandle, an owned value that, 
        when join method is called on it, will wait for its thread 
        
    move keyword with closures passed to thread::spawn takes ownership 
        of the vals it uses from the environment (one thread to another); 
        to use data from the main thread in the spawned thread, 
        the spawned thread's closure must capture the vals it needs 

    Ex. Rust INFERS how to capture v without move, and because prinln! 
        only needs a ref to v, the closure tries to borrow v; but Rust 
        cannot tell how long the spawned thread will run, so it does not 
        know if the ref to v will always be valid (drop(v) in the scope)  
        
*/

use std::thread; 
use std::time::Duration; 

fn main() {
    let handle = thread::spawn(|| { 
        // code to be run in the new thread 
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!"); 
            thread::sleep(Duration::from_millis(1)); 
        }
    }); 

    // handle.join().unwrap(); at this point will run 
    // the spawned tread first (will not be interleaved)  

    for i in 1..5 {
        println!("hi number {i} from the main thread!"); 
        thread::sleep(Duration::from_millis(1));  
    }

    // blocks the thread currently running until 
    // thread represented by the handle terminates 
    handle.join().unwrap(); 

    let v = vec![1, 2, 3]; 
    
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}"); 
    }); 

    // drop(v); will mave v not valid anymore 

    handle.join().unwrap(); 
}
