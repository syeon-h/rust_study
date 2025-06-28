/*
    turining a single-threaded server into a multithreaded server 
        with single-threaded, the server will process each req in turn, 
        thus, as it receives more and more reqs, the serial execution 
        would be less and less optimal 

    thread pool is a group of spawned threads that are waiting and ready 
        to handle a task; when the program receives a new task, it assigns 
        one of the threads in the pool to the task 

        when the first is done prcocessing its task, it it returned to the 
        pool of idle threads, ready to handle a new one 

        thus, process connections concurrently, increasing the throughput 


*/

use std::{
    fs, 
    io::{prelude::*, BufReader}, 
    net::{TcpListener, TcpStream}, 
    thread, 
    time::Duration, 
}; 

use hello::ThreadPool; 

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); 
    let pool = ThreadPool::new(4); 

    // to accept only two reqs before gracefully shutting down: 
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap(); 

        // takes the closure and gives it to a thread in the pool
        pool.execute(|| {
            handle_connection(stream); 
        }); 
    }

    println!("Shutting down.")
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream); 
    // collect the lines of the req the browser sends 
    // let http_request: Vec<_> = buf_reader 
    //    .lines() 
    //    .map(|result| result.unwrap()) 
    //    .take_while(|line| !line.is_empty()) // till empty string 
    //    .collect(); 
    let request_line = buf_reader.lines().next().unwrap().unwrap(); 
        // only look at the first line of the HTTP req 
        // first unwrap takes care of the Option 
        // second unwrap handles the Result 


    // println!("Request: {http_request:#?}"); 

    // #3: response that uses HTTP version 1.1 
    // let response = "HTTP/1.1 200 OK\r\n\r\n"; 

    // #5: handling requests to / 
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            // server will sleep for 5 secs 
            thread::sleep(Duration::from_secs(5)); 
            ("HTTP/1.1 200 OK", "hello.html") 
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    }; 

    let contents = fs::read_to_string(filename).unwrap(); 
    let length = contents.len(); 

    // #4: sending hello.html as the body of the response 
    let response = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"); 

    stream.write_all(response.as_bytes()).unwrap(); 
}
