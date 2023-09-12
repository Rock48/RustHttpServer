use std::io::{
    ErrorKind, 
    Result as IoResult
};
use std::fs;
use super::http::{
    RequestHandler,
    Request,
    Response,
};

pub struct WebsiteHandler {
    public_dir: String
}

impl WebsiteHandler {
    pub fn new(public_dir: String) -> Self {
        Self { public_dir }
    }
    fn read_file(&self, file_path: &str) -> IoResult<String> {
        let path = format!("{}/{}", self.public_dir, file_path);

        match fs::canonicalize(&path) {
            Ok(pb) => {
                if pb.starts_with(&self.public_dir) {
                    fs::read_to_string(pb)
                } else {
                    Err(err!(PermissionDenied, "Directory traversal attack attempted", "Attempted Path: {}", path))
                }
            },
            Err(e) => Err(e)
        }
    }
}

impl RequestHandler for WebsiteHandler {

    fn get(&self, req: &Request, res: &mut Response) -> IoResult<()> {
        match req.path() {
            "/" => {
                match self.read_file("index.html") {
                    Ok(body) => {
                        res.ok(Some(body))
                    },
                    Err(e) => {
                        res.gen_404().append(format!("<p>{}</p>",e.to_string())).send()
                    }
                }
            },
            "/apples" => res.gen_404().append(Str!("We only have bananas")).send(),
            path => {
                match self.read_file(path) {
                    Ok(body) => res.ok(/*Now you're just*/Some(body)/*That I used to know*/),
                    Err(e) => {
                        if e.kind() == ErrorKind::PermissionDenied {
                            return res.send_403();
                        }
                        res.send_404()
                    }
                }
            }
        }
    }

    fn post(&self, _req: &Request, res: &mut Response) -> IoResult<()> {
        res.gen_404().append(Str!("We don't recieve mail")).send()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_index() {
        let handler = WebsiteHandler::new(format!("{}/public", env!("CARGO_MANIFEST_DIR")));
        match handler.read_file("index.html") {
            Ok(s) => assert!(s.len() > 0),
            Err(e) => panic!("Error reading index.html file! {}", e),
        }
    }
    
    #[test]
    fn file_not_found() {
        let handler = WebsiteHandler::new(format!("{}/public", env!("CARGO_MANIFEST_DIR")));
        match handler.read_file("invalid_file") {
            Err(e) => assert_eq!(e.kind(), ErrorKind::NotFound),
            Ok(_) => panic!("invalid_file read returned Ok"),
        }
    }
    #[test]
    fn directory_transversal() {
        let handler = WebsiteHandler::new(format!("{}/public", env!("CARGO_MANIFEST_DIR")));
        match handler.read_file("../src/main.rs") {
            Err(e) => assert_eq!(e.kind(), ErrorKind::PermissionDenied),
            Ok(_) => panic!("DIRECTORY TRANSVERSAL ATTACK WAS SUCCESSFUL"),
        }
    }
}