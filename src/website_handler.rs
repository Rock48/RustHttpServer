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

    // uses src/http as public so it doesn't rely on any actual files being in the public dir
    #[test]
    fn read_existing_file() {
        let handler = WebsiteHandler::new(format!("{}/src/http", env!("CARGO_MANIFEST_DIR")));
        match handler.read_file("server.rs") {
            Ok(s) => assert!(s.len() > 0),
            Err(e) => panic!("Error reading server.rs file! {}", e),
        }
    }
    
    #[test]
    fn file_not_found() {
        let handler = WebsiteHandler::new(format!("{}/src/http", env!("CARGO_MANIFEST_DIR")));
        match handler.read_file("invalid_file") {
            Err(e) => assert_eq!(e.kind(), ErrorKind::NotFound),
            Ok(_) => panic!("invalid_file read returned Ok"),
        }
    }
    #[test]
    fn directory_traversal() {
        let handler = WebsiteHandler::new(format!("{}/src/http", env!("CARGO_MANIFEST_DIR")));
        match handler.read_file("../main.rs") {
            Err(e) => assert_eq!(e.kind(), ErrorKind::PermissionDenied),
            Ok(_) => panic!("DIRECTORY TRAVERSAL ATTACK WAS SUCCESSFUL"),
        }
    }
}