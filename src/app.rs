use crate::network::url::URL;

pub fn init()
{
    println!("a");
    let a = String::from("https://www.google.com");
    let b = String::from("http://www.google.com");
    let c = String::from("twitch.tv");

    let url = URL::init(&a);
    println!("{:?}", url.scheme);
    let url = URL::init(&b);
    println!("{:?}", url.scheme);
    let url = URL::init(&c);
    println!("{:?}", url.scheme);
}
