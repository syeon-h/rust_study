/*
    two main protocols involved in web servers are (1) HTTP and (2) TCP 
        TCP describes the details of how information gets from one server 
        to another but does not specify what that information is 

        HTTP builds on top of TCP by defining the contents

    step#1: listen to the TCP connection 
        bind fn returns a Result<T, E>, thus, it is possible to fail 

        incoming method returns an iterator with a sequence of streams; 
        a single stream represents an open connection btw the client and 
        the server 
        
        connection is the name for the full req and res process: 
            1. a client connects to the server 
            2. the server generates a response 
            3. the server closes the connection; 
        read from the TcpStream to see what the client sent and then write 
        our response to the stream and send back to the client 

    step#2: reading the request 
        to separate the concerns of first getting a connection and then 
        taking some action with the connection 

        browser signals the end of an HTTP req by sending two newlines, so
        take lines until getting the empty string 

        we get multiple connections from one browser req because if the
        repeated connections are all requesting /, we know the browser is 
        trying to fetch / repeatedly because it is not getting a response 

        HTTP is a text-based protocol, and a req takes this format: 
            Method Request-URI HTTP-Version CRLF (request line)
            headers CRLF (Host: onward)
            message-body (GET have no body) 

        req line holds info about the client's req; Request-URI is /, 
        the Uniform Resource Identifier (URI) the client is requesting; 
        HTTP-Version is the one client uses, and then ends in a CRLF seq 
        (separates the req line from the rest and can be written as \r\n) 

    step#3: writing a response 
        responses have the following format: 
            HTTP-Version Status-Code Reason-Phrase CRLF (status line) 
            headers CRLF (if any) 
            message-body 
        status line contains the HTTP version used, a numeric status code 
        that summarizes the result, and a reason phrase that provides a 
        text description of the status code (ex. HTTP/1.1 200 OK\r\n\r\n) 

    step#4: returning real HTML 
        use format! to add the file's contents as the body of the success 
        response; to ensure, add the Content-Length header which is set to 
        the size of the response body (hello.html) 

    step#5: validating the req and selectively responding 
        add functionality to check that the browser is requesting / before 
        returning the HTML file and return an error for anything else 
*/

use std::{
    fs, 
    io::{prelude::*, BufReader}, 
    net::{TcpListener, TcpStream}, 
}; 

fn main() {
    // #1: listen to the TCP connection 
    // bind works like the new function; return a new TcpListner instance 
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); 

    for stream in listener.incoming() {
        let stream = stream.unwrap(); 

        //println!("Connection established!"); 
        handle_connection(stream); 
    }
}

// #2: reading the request 
// read data from the TCP stream and print it 
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
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html") 
    }; 

    let contents = fs::read_to_string(filename).unwrap(); 
    let length = contents.len(); 

    // #4: sending hello.html as the body of the response 
    let response = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"); 

    stream.write_all(response.as_bytes()).unwrap(); 
}
