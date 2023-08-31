use std::io::Result as IoResult;

use super::{ Method, Request, Response };

pub trait RequestHandler { 
    fn handle(&self, req: &Request, res: &mut Response) {
        let result = match req.method() {
            Method::GET => self.get(req, res),
            Method::POST => self.post(req, res),
            Method::DELETE => self.delete(req, res),
            Method::PUT => self.put(req, res),
            Method::HEAD => self.head(req, res),
            Method::CONNECT => self.connect(req, res),
            Method::OPTIONS => self.options(req, res),
            Method::TRACE => self.trace(req, res),
            Method::PATCH => self.patch(req, res),
        };

        if let Err(e) = result {
            println!("Sending response failed with error {}", e);
        }
    }

    fn handle_bad(&self, res: &mut Response, body: &str) {
        if let Err(e) = res.bad_request(Some(format!("<h1>400 Bad Request</h1><p>{}</p>", body))) {
            println!("Sending 400 response failed with error {}", e);
        }
    }
    
    fn get(&self, _req: &Request, res: &mut Response) -> IoResult<()> { res.send_404() }
    fn delete(&self, _req: &Request, res: &mut Response) -> IoResult<()> { res.send_404() }
    fn post(&self, _req: &Request, res: &mut Response) -> IoResult<()> { res.send_404() }
    fn put(&self, _req: &Request, res: &mut Response) -> IoResult<()> { res.send_404() }
    fn head(&self, _req: &Request, res: &mut Response) -> IoResult<()> { res.send_404() }
    fn connect(&self, _req: &Request, res: &mut Response) -> IoResult<()> { res.send_404() }
    fn options(&self, _req: &Request, res: &mut Response) -> IoResult<()> { res.send_404() }
    fn trace(&self, _req: &Request, res: &mut Response) -> IoResult<()> { res.send_404() }
    fn patch(&self, _req: &Request, res: &mut Response) -> IoResult<()> { res.send_404() } 
}