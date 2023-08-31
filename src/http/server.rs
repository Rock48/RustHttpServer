use std::io::Read;
use std::net::TcpListener;
use super::{Request, Response, RequestHandler};

pub struct Server {
    ip: String,
    port: u16,
    listener: TcpListener
}

impl Server {
    pub fn new(ip: String, port: u16) -> Self {
        
        Server {
            listener: TcpListener::bind(format!("{}:{}", &ip, port)).expect("Port is already in use"),
            ip,
            port,
        }
    }

    pub fn run(&mut self, handler: impl RequestHandler) {
        println!("Listening on {}", self.addr());

        loop { 
            let mut stream = match self.listener.accept() {
                Err(e) => {
                    println!("Failed to establish a connection {}", e);
                    continue;
                },
                Ok((stream, _)) => stream,
            };

            let mut bytes = [0; (2 as usize).pow(10)];
            let read_result = stream.read(&mut bytes);
            let mut response = Response::new(&mut stream);

            if let Err(e) = read_result {
                println!("Failed to read request bytes {}", e);
                handler.handle_bad(&mut response, "Failed to read request bytes");
                continue;
            }
            
            // &bytes[..] creates a byte slice with the entire array
            let request = match Request::try_from(&bytes[..]) {
                Ok(request) => request,
                Err(e) => {
                    println!("Error converting bytes to result: {}", e);
                    handler.handle_bad(&mut response, "Error converting bytes to result");
                    continue;
                }
            };
            println!("Recieved a request: {:?}", request);
            
            handler.handle(&request, &mut response);
        }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

