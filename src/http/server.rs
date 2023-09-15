use std::{
    io::{Read, Result as IoResult},
    net::{TcpListener, TcpStream},
    rc::Rc,
    cell::RefCell,
    thread,
    num::NonZeroUsize,
    sync::Arc
};
use rayon::{ThreadPoolBuilder, ThreadPool};

use super::{Request, Response, RequestHandler};

pub struct Server {
    ip: String,
    port: u16,
    listener: TcpListener,
    thread_pool: ThreadPool
}

impl Server {
    pub fn new(ip: String, port: u16) -> Self {
        let thread_count = thread::available_parallelism().unwrap_or(NonZeroUsize::new(1).unwrap()).get();
        let thread_pool = ThreadPoolBuilder::new().num_threads(thread_count).build().expect("Thread pool failed to build!!!");
        Server {
            listener: TcpListener::bind(format!("{}:{}", &ip, port)).expect("Port is already in use"),
            ip,
            port,
            thread_pool
        }
    }

    pub fn run(&mut self, handler: Arc<impl RequestHandler + Send + Sync + 'static>) {
        println!("Listening on {}", self.addr());

        loop { 
            let stream = match self.listener.accept() {
                Err(e) => {
                    eprintln!("Failed to establish a connection {}", e);
                    continue;
                },
                Ok((stream, _)) => stream,
            };

            self.spawn_thread(&handler, stream);
        }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    fn spawn_thread(&self, handler: &Arc<impl RequestHandler + Send + Sync + 'static>, stream: TcpStream) {
        let handler = handler.clone();
        self.thread_pool.spawn(move || {
            let stream = Rc::new(RefCell::new(stream));
            let mut bytes = [0; (2 as usize).pow(10)];
            let mut response = Response::new(stream.clone());
            let result = match Self::decode(stream.clone(), &mut bytes) {
                Ok(mut req) => handler.handle(&mut req, &mut response),
                Err(err) => handler.handle_bad(&mut response, &err.to_string())
            };

            if let Err(e) = result {
                eprintln!("Something went wrong sending response:{}\n{:?}", e, response);
            }
        })
    }

    fn decode(stream: Rc<RefCell<TcpStream>>, bytes: &mut [u8]) -> IoResult<Request> {
        let read_result = stream.borrow_mut().read(bytes);
        if let Err(e) = read_result {
            eprintln!("Failed to read request bytes {}", e);
            return Err(err!(InvalidData, "Failed to read request bytes"))
        }
        
        // &bytes[..] creates a byte slice with the entire array
        let request = match Request::try_from(&bytes[..]) {
            Ok(request) => Ok(request),
            Err(e) => {
                eprintln!("Error converting bytes to result: {}", e);
                Err(err!(InvalidData, "Error converting bytes to result!"))
            }
        }?;
        println!("Recieved a request: {:?}", request);
        
        Ok(request)
    }
}

