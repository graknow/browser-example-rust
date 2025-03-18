use std::net::TcpStream;

use crate::network::{http::{self, request}, url::URL};

pub fn init()
{
    let a = String::from("http://www.google.com:80/");
    let b = String::from("http://www.google.com/search?term=test");
    let c = String::from("twitch.tv");

    let url = URL::init(&a);
    println!("Scheme: {:?}, Host: {}, Path: {}, Port: {}", url.scheme, url.host, url.path, url.port);

    request(url, http::Method::GET);
}
