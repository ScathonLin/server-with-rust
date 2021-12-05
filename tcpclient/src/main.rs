use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
fn main() {
    let mut s = TcpStream::connect("127.0.0.1:8090").unwrap();
    s.write("Hello".as_bytes()).unwrap();

    let mut buffer = [0; 5];
    s.read(&mut buffer).unwrap();
    println!("Read from server: {:?}", str::from_utf8(&buffer).unwrap())
}
