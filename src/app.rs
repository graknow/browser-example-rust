use crate::network::{
    url::URL
};

pub fn init() {
    let a = String::from("http://example.org");
    let b = String::from("http://www.google.com/search?term=test");
    let c = String::from("twitch.tv");

    let url = URL::init(&a);
    println!(
        "Scheme: {:?}, Host: {}, Path: {}, Port: {}",
        url.scheme, url.host, url.path, url.port
    );

    let mut response: Response = Response { data: String::from("") };

    url.request();
    eprintln!("{}", response.data);
}
