/*
    ThreadPool implementation will be independent of the web server 
        limit the number of threads to protect us from DoS attack 

        new function needs to have one param that can accept 4 as an arg 
        and should return a ThreadPool instance 

        execute method takes a closure as a param; as three traits that 
        can take closures as params are Fn, FnMut, and FnOnce; to decide 
        from these, look up the similar thread::spawn implementation: 
            pub fn spawn<F, T>(f: F) -> JoinHandle<T>
                where
                    F: FnOnce() -> T,
                    F: Send + 'static,
                    T: Send + 'static,
        F type param is the one we are concerned as the T type param is 
        related to the return val and spwan is using FnOnce as the trait; 
        this is the one we want as well, becasue we will eventually pass 
        the arg we get in execute to spawn 

        F type param also has the trait bound Send and the lifetime bound:  
        need Send to transfer the closure from one thread to another and 
        'static because how long the thread will take is not known 

        use Arc<Mutex<T>> to share ownership across multiple threads; 
        Arc type will let multiple workers own the receiver, and Mutex 
        will ensure that only one gets a job from the receiver at a time 

        change Job from a struct to a type alias for a trait object that 
        holds the type of closure that execute receives 

        let job = receiver.lock().unwrap().recv().unwrap(); works because 
        with let, any temporary vals used on the rhs are immediately 
        dropped when the let statement ends; however, while let does not 
        until the end of the associated block (thus it does not work) 
    
    graceful shutdown and cleanup: 

*/
use std::{
    sync::{mpsc, Arc, Mutex}, 
    thread::{self, Thread}, 
}; 

pub struct ThreadPool {
    workers: Vec<Worker>, 
    sender: Option<mpsc::Sender<Job>>, 
}

// channel to function as the queue of jobs 
// struct Job; 
type Job = Box<dyn FnOnce() + Send + 'static>; 

impl ThreadPool {
    /// Create a new ThreadPool 
    /// 
    /// The size is the number of threads in the pool. 
    /// 
    /// # Panics 
    /// 
    /// The `new` function will panic if the size is zero. 
    pub fn new(size: usize) -> ThreadPool {
        // validate the # of threads 
        assert!(size > 0); 

        let (sender, receiver) = mpsc::channel(); 
        
        // share the receiver among the workers 
        let receiver = Arc::new(Mutex::new(receiver)); 

        let mut workers = Vec::with_capacity(size); 

        // create a new Worker with an id 
        // and store the worker in the vector 
        for id in 0..size {
            // pass the receiver to the workers 
            workers.push(Worker::new(id, Arc::clone(&receiver))); 
        }

        ThreadPool { 
            workers, 
            sender: Some(sender), 
        }
    }

    pub fn execute<F>(&self, f: F) 
    where 
        // () after FnOnce because it represents a closure 
        // that takes no param and returns the unit type () 
        F: FnOnce() + Send + 'static, 
    {
        let job = Box::new(f); 

        // send the job down the channel 
        self.sender.as_ref().unwrap().send(job).unwrap(); 
    }
}

// picks up code that needs to be run and 
// runs the code in the Worker's thread 
struct Worker {
    id: usize, 
    thread: Option<thread::JoinHandle<()>>, 
        // take on Option takes the Some variant out 
        // and leaves None in its place (to clean up) 
}

impl Worker {
    // takes an id number and return a Worker instance 
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // pass the receiver to the workers 
            let message = receiver.lock().unwrap().recv(); 
                // lock to acquire the mutex (blocks recv if no job) 
                // then recv to receive a Job from the channel 

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing."); 

                    // execute the jobs in the worker's thread 
                    // lock remains held for the duration of the call to job() 
                    job(); 
                }
                // explicitly break out of the loop when recv returns Err 
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down."); 
                    break; 
                }
            }
        }); 

        Worker { 
            id, 
            thread: Some(thread), 
        }
    }
}


impl Drop for ThreadPool {
    fn drop(&mut self) {
        // explicitly drop sender (closes the channel) 
        // calls to recv in the infinite loop will return Err 
        drop(self.sender.take()); 

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id); 

            // joining each thread when the pool goes out of scope 
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap(); 
            }
        }
    }
}