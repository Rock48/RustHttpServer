use super::StatusCode;
use std::{
    io::{ Write, Result as IoResult},
    fmt::{
        Formatter, Debug, Display,
        Result as FmtResult
    },
};

pub struct Response<'w> {
    pub status: StatusCode,
    pub body: Option<String>,
    writer: &'w mut dyn Write
}

impl<'w> Display for Response<'w> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let body = self.body.as_deref().unwrap_or("");
        write!(f, "Status: {}, Body: {}", self.status, body)
    }
}
impl<'w> Debug for Response<'w> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Response {{ Status: {:?}, Body: {:?} }}", self.status, self.body)
    }
}

impl<'w> Response<'w> {
    /**
     * 
     * Creates a new writer with an empty body */
    pub fn new(writer: &'w mut impl Write) -> Self {
        Self { status: StatusCode::Ok, body: None, writer }
    }
    
    pub fn writer(&self) -> & dyn Write { self.writer }

    pub fn ok(&mut self, body: Option<String>) -> IoResult<()> {
        self.status = StatusCode::Ok;
        self.body = body;
        self.send()
    }
    pub fn bad_request(&mut self, body: Option<String>) -> IoResult<()> {
        self.status = StatusCode::BadRequest;
        self.body = body;
        self.send()
    }

    pub fn not_found(&mut self, body: Option<String>) -> IoResult<()> {
        self.status = StatusCode::NotFound;
        self.body = body;
        self.send()
    }

    
    /** Sends a 404 with a generic HTML body */
    pub fn gen_404(&mut self) -> &mut Self {
        self.status = StatusCode::NotFound;
        self.body = some_str!("<h1>404 Not Found</h1><p>The page you requested could not be found on this server.</p>");
        self
    }
    
    /// Generates a 404 using [Response::gen_404()] and sends it
    pub fn send_404(&mut self) -> IoResult<()> {
        self.gen_404().send()
    }

    /// If [self.body](Option<String>) is [Some] then appends [str](String) to it, otherwise
    /// if it simply sets it to [str](String)
    pub fn append(&mut self, str:String) -> &mut Self {
        self.body = match &self.body {
            Some(cur) => Some(format!("{}{}", cur, str)),
            None => Some(str)
        };
        self
    }

    pub fn send(&mut self) -> IoResult<()> {
        let body = self.body.as_deref().unwrap_or("");

        write!(self.writer,"HTTP/1.1 {} {}\r\nTODO: HEADERS\r\n\r\n{}",self.status.code(),self.status,body)
    }
}