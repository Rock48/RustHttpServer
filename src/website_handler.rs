use std::io::Result as IoResult;
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
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_dir, file_path);

        match fs::canonicalize(&path) {
            Ok(pb) => {
                if pb.starts_with(&self.public_dir) {
                    fs::read_to_string(pb).ok()
                } else {
                    println!("Directory traversal attack attempted: {}", &path);
                    None
                }
            },
            Err(_) => None
        }
    }
}

impl RequestHandler for WebsiteHandler {

    fn get(&self, req: &Request, res: &mut Response) -> IoResult<()> {
        match req.path() {
            "/" => {
                let result = self.read_file("index.html");
                if(result.is_some()) {
                    res.ok(result)
                } else {
                    res.send_404()
                }
            },
            "/apples" => res.gen_404().append(Str!("We only have bananas")).send(),
            path => {
                match self.read_file(path) {
                    Some(body) => res.ok(/*Now you're just*/Some(body)/*That I used to know*/),
                    None => {
                        
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