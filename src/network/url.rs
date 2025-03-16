#[derive(Debug)]
pub enum Scheme {
    HTTP,
    HTTPS,
}

pub struct URL {
    pub scheme: Scheme
}

impl URL {
    pub fn init(url: &String) -> Self {
        Self {
            scheme: Self::parse_scheme(url)
        }
    }

    fn parse_scheme(url: &String) -> Scheme {
        let scheme_string: &str = match url.split("://").next() {
            Some(x) => x,
            None => ""
        };

        match scheme_string {
            "http" => Scheme::HTTP,
            "https" => Scheme::HTTPS,
            _ => Scheme::HTTPS
        }
    }
}
