use core::fmt;
use std::{
    io::{
        Read, Write
    }, 
    net::{TcpStream, ToSocketAddrs}
};

use crate::network::url;

pub enum Method {
    GET,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
        }
    }
}

pub fn request(url: url::URL, method: Method) {
    let mut request: String = String::from("");

    request += &format!("{} {} HTTP/1.0\r\n", method, url.path);
    request += &format!("Host: {}\r\n", url.host);
    request += "\r\n";

    let mut stream: TcpStream = 
        TcpStream::connect(format!("{}:{}", url.host, url.port))
        .expect("Failed to create connection.");
    let _ = stream.write(&request.as_bytes());

    let mut rx_bytes = [0u8; 100];
    // Read from the current data in the TcpStream
    let _ = stream.read(&mut rx_bytes);

    let received = std::str::from_utf8(&rx_bytes).expect("valid utf8");
    eprintln!("{}", received);
}