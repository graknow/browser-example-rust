use core::fmt;
use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

const HTTP_DEFAULT_PORT: u16 = 80;
const HTTPS_DEFAULT_PORT: u16 = 443;

#[derive(Debug)]
pub enum Scheme {
    HTTP,
    HTTPS,
}

pub struct URL {
    pub scheme: Scheme,
    pub host: String,
    pub port: u16,
    pub path: String,
}

impl URL {
    pub fn new(url: &String) -> Self {
        let (scheme_string, address): (&str, &str) =
            url.split_once("://").unwrap_or_else(|| ("", url));

        let scheme: Scheme = match scheme_string {
            "http" => Scheme::HTTP,
            "https" => Scheme::HTTPS,
            _ => Scheme::HTTPS,
        };

        let host: &str;
        let path: &str;
        let port: u16;

        if !address.is_empty() {
            let host_full: &str;
            (host_full, path) = match address.find('/') {
                Some(x) => address.split_at_checked(x).unwrap_or_else(|| ("", "/")),
                None => (address, "/"),
            };

            (host, port) = match host_full.find(':') {
                Some(_) => {
                    let (h, p) = host_full.split_once(':').unwrap();
                    (h, p.parse().unwrap())
                }
                None => (
                    host_full,
                    match scheme {
                        Scheme::HTTP => HTTP_DEFAULT_PORT,
                        Scheme::HTTPS => HTTPS_DEFAULT_PORT,
                    },
                ),
            }
        } else {
            host = "";
            path = "";
            port = HTTP_DEFAULT_PORT;
        }

        Self {
            scheme: match scheme_string {
                "http" => Scheme::HTTP,
                "https" => Scheme::HTTPS,
                _ => Scheme::HTTPS,
            },
            host: String::from(host),
            port: port,
            path: String::from(path),
        }
    }
}
