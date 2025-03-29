pub const MAX_RESPONSE_LENGTH: usize = 4096;

pub enum Response {
    HTTP(HttpResponse),
}

pub struct HttpResponse {
    pub secure: bool,
    pub 
}