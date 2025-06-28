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



*/

use std::net::TcpListener; 

fn main() {
    // #1: listen to the TCP connection 
    // bind works like the new function; return a new TcpListner instance 
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); 

    for stream in listener.incoming() {
        let stream = stream.unwrap(); 

        println!("Connection established!"); 
    }
}
