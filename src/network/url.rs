const HTTP_DEFAULT_PORT: u16 = 80;
const HTTPS_DEFAULT_PORT: u16 = 443;

#[derive(Debug)]
pub enum UrlError {
    MissingScheme,
    InvalidScheme,
    MissingHost,
    InvalidPort,
}

#[derive(Debug, PartialEq)]
pub enum UrlScheme {
    HTTP,
    HTTPS,
    FILE,
}

const VALID_SCHEMES: [&str; 3] = [
    "http", 
    "https", 
    "file"
];

pub struct Url {
    // A url will be broken down as follows.
    // scheme :// host [: port]? / path [? query]?
    // This is a simpler breakdown than other Url implementations since
    // as this application is just for learning.
    full: String,
    scheme_end: usize,
    host_start: usize,
    host_end: usize,
    port_end: Option<usize>,
    path_start: usize,
    query_start: Option<usize>
}

impl Url {
    /// Create a URL struct instance.
    pub fn new(url: &str) -> Result<Url, UrlError> {
        let mut result = Url {
            full: String::from(url),
            scheme_end: 0,
            host_start: 0,
            host_end: 0,
            port_end: None,
            path_start: 0,
            query_start: None,
        };

        result.scheme_end = match result.full.find("://") {
            Some(x) => x,
            None => return Err(UrlError::MissingScheme),
        };

        assert!(result.scheme_end > 0);
        
        if !VALID_SCHEMES.contains(&&result.full[..result.scheme_end]) {
            return Err(UrlError::InvalidScheme);
        }

        result.host_start = result.scheme_end + 3;

        if let Some(full_host_end) = result.full[result.host_start..].find("/") {
            result.path_start = full_host_end + result.host_start;
            match result.full[result.host_start..full_host_end].find(":") {
                Some(x) => {
                    result.port_end = Some(full_host_end + result.host_start);
                    result.host_end = x + result.host_start;
                },
                None => result.host_end = full_host_end + result.host_start,
            };
        }
        else {
            return Err(UrlError::MissingHost);
        }


        assert!(result.host_end > 0);
        assert!(result.scheme_end < result.host_end);

        result.query_start = result.full.find("?");

        Ok(result)
    }

    pub fn full(&self) -> &str {
        &self.full
    }

    pub fn scheme(&self) -> UrlScheme {
        match &self.full[0..self.scheme_end] {
            "http" => UrlScheme::HTTP,
            "https" => UrlScheme::HTTPS,
            "file" => UrlScheme::FILE,
            _ => panic!("Invalid scheme for {}", self.full),
        }
    }

    pub fn scheme_str(&self) -> &str {
        &self.full[0..self.scheme_end]
    }

    pub fn host(&self) -> &str {
        &self.full[self.host_start..self.host_end]
    }

    pub fn port(&self) -> Option<&str> {
        match self.port_end {
            Some(x) => Some(&self.full[self.host_end..x]),
            None => None,
        }
    }

    pub fn path(&self) -> &str {
        let path_end = self.query_start.unwrap_or_else(|| self.full.len());
        &self.full[self.path_start..path_end]
    }

    pub fn query(&self) -> Option<&str> {
        match self.query_start {
            Some(x) => Some(&self.full[x..]),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_http() {
        let url: Url = Url::new("https://www.google.com/search?query=test+query")
            .expect("String was a valid Url");

        assert_eq!(url.scheme_str(), "https");
        assert_eq!(url.scheme(), UrlScheme::HTTPS);
        assert_eq!(url.host(), "www.google.com");
        assert_eq!(url.port(), None);
        assert_eq!(url.path(), "/search");
        assert_eq!(url.query(), Some("?query=test+query"));
    }
}
