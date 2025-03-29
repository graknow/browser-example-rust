use crate::display::html;
use crate::network::{http, url::URL};
pub fn init() {
    let a = String::from("http://example.org");
    let b = String::from("http://www.google.com");
    let c = String::from("http://browser.engineering/examples/example1-simple.html");

    let url = URL::new(&c);

    let response = http::request(url);
    html::show(response.body.as_str());
}
