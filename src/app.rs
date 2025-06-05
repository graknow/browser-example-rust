use crate::network::url::Url;

pub fn init() {
    let a = String::from("http://example.org");
    let b = String::from("http://www.google.com");
    let c = String::from("https://browser.engineering/examples/example1-simple.html");

    let url: Url = Url::new(&c).expect("");

    //let response = http::request(url);
    //html::show(response.body.as_str());
}
