use super::url::Url;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

pub const HTTP_MAX_RESPONSE_LENGTH: usize = 4096;

#[derive(Debug)]
pub enum HttpVersion {
    HTTP0_9,
    HTTP1_0,
    HTTP1_1,
}

pub struct HttpResponse {
    pub version: HttpVersion,
    pub status: u16,
    pub explanation: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

pub fn request(url: Url) -> HttpResponse {
    let mut request: String = String::from("");

    request += &format!("{} {} HTTP/1.0\r\n", "GET", url.path());
    request += &format!("Host: {}\r\n", url.host());
    request += "\r\n";

    let mut stream: TcpStream = TcpStream::connect(format!("{}:{}", url.host(), url.port().unwrap()))
        .expect("Failed to create connection.");
    let _ = stream.write(&request.as_bytes());

    let mut rx_bytes = [0u8; HTTP_MAX_RESPONSE_LENGTH];
    // Read from the current data in the TcpStream
    let _ = stream.read_exact(&mut rx_bytes);
    let mut data = str::from_utf8(&rx_bytes).expect("Valid UTF-8").lines();

    let status_values = data
        .next()
        .expect("At least one response line")
        .splitn(3, " ")
        .collect::<Vec<&str>>();

    assert_eq!(status_values.len(), 3);

    let mut headers: HashMap<String, String> = HashMap::new();

    loop {
        let line = data.next().unwrap_or_else(|| "");

        if line.eq("") {
            break;
        }

        let (header, value) = line.split_once(": ").unwrap_or_else(|| ("", ""));

        if header.is_empty() || value.is_empty() {
            continue;
        }

        headers.insert(String::from(header).to_lowercase(), String::from(value));
    }

    assert!(
        !headers.contains_key("transfer-encoding"),
        "Shouldn't contain header \"transfer-encoding\"."
    );
    assert!(
        !headers.contains_key("content-encoding"),
        "Shouldn't contain header \"content-encoding\"."
    );

    let data = data.collect::<Vec<&str>>();
    let body_start = data.iter().position(|s| s.contains("<body>")).unwrap();
    let body_end = data.iter().position(|s| s.contains("</body>")).unwrap();

    HttpResponse {
        version: match status_values[0] {
            "HTTP/0.9" => HttpVersion::HTTP0_9,
            "HTTP/1.0" => HttpVersion::HTTP1_0,
            "HTTP/1.1" => HttpVersion::HTTP1_1,
            _ => HttpVersion::HTTP0_9,
        },
        status: status_values[1]
            .parse::<u16>()
            .expect("Valid u16 status code"),
        explanation: String::from(status_values[2]),
        headers: headers,
        body: String::from(data[body_start..=body_end].join("\r\n")),
    }
}
