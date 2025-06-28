/*
    message paassing is where threads or actors communicate 
        by sending each other messages containing data and to accomplish 
        message-sending concurrency, Rust provides channels

    channel is a general programming concept by which data is sent from 
        one thread to another like a directional channel of water (stream); 
        channel has two halves: a transmitter (tx) and a receiver (rx); 
        channel is closed if either the tx or rx half is dropped 

    mpsc::channel function create a new channel; mpsc stands for 
        multiple produce, single consumer, thus, Rust's channel can have 
        multiple sending ends but only one receiving end that consumes 

        returns a tuple that the first element is the sending end (tx) 
        and the second is the receiving end (rx) 

        recv (receive) will block the main thread's execution and wait 
        until a val is sent down; when tx closes, will return an error 

        try_recv does not block, but return a Result<T, E> immediately; 
        useful if this thread has other work to do while waiting for 
        messagess (ex. write a loop that calls try_recv every so often) 

    ownership rule prevents error in concurrent programming 
        Ex. use a val in the spawned thread after it is sent it down 
        this concurrency mistake will cause a compile time error 

    
        

*/

use std::sync::mpsc; 
use std::thread; 
use std::time::Duration; 

fn main() {
    // create a channel and assign the two halves: 
    let (tx, rx) = mpsc::channel(); 

    // moving tx to a spawned thread 
    // send method returns a Result<T E> type 
    thread::spawn(move || {
        // let val = String::from("hi"); 

        // sending multiple vals 
        let vals = vec![
            String::from("hi"), 
            String::from("from"), 
            String::from("the"), 
            String::from("thread"), 
        ]; 

        // tx.send(val).unwrap(); 

        for val in vals {
            tx.send(val).unwrap(); 
            thread::sleep(Duration::from_secs(1)); 
        }
        // println!("val is {val}"); will not work 
    }); 

    // receiving the value 
    // let received = rx.recv().unwrap(); 

    // iteration end when the channel is closed 
    for received in rx {
        // main thread is waiting to receive vals 
        println!("Got: {received}"); 
    }

    // creating multiple producers by cloning the tx: 
    let (tx, rx) = mpsc::channel(); 

    // new tx that can be passed to the first spawned thread 
    let tx1 = tx.clone(); 
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"), 
            String::from("from"), 
            String::from("the"), 
            String::from("thread"), 
        ]; 

        for val in vals {
            tx1.send(val).unwrap(); 
            thread::sleep(Duration::from_secs(1)); 
        }
    }); 

    // original tx to a second spawned thread 
    thread::spawn(move || {
        let vals = vec![
            String::from("more"), 
            String::from("messages"), 
            String::from("for"), 
            String::from("you"), 
        ]; 

        for val in vals {
            tx.send(val).unwrap(); 
            thread::sleep(Duration::from_secs(1)); 
        }
    }); 

    // two threads that each sends different messages 
    // order is nondeterministic and create different output each time 
    for received in rx {
        println!("Got: {received}"); 
    }
}
