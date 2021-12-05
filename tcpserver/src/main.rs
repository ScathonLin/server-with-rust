use std::{net::TcpListener};
use std::io::{Read, Write};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8090").unwrap();
    println!("Running on port 8090...");
    for stream in listener.incoming() {
        let mut s = stream.unwrap();
        let mut buf = [0; 1024];
        s.read(&mut buf).unwrap();
        s.write(&mut buf).unwrap();
        println!("connection established.");
    }
}
