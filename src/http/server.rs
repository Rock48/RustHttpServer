use std::{
    io::{Read, Result as IoResult},
    net::{TcpListener, TcpStream},
    rc::Rc,
    cell::RefCell, thread::{self, JoinHandle}, num::NonZeroUsize, sync::Arc
};
use crossbeam_channel::{bounded, Receiver};

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

    pub fn run(&mut self, handler: Arc<impl RequestHandler + Send + Sync + 'static>) {
        println!("Listening on {}", self.addr());

        let (send, recv) = bounded::<TcpStream>(100);
        // I hate this line
        let thread_count = thread::available_parallelism().unwrap_or(NonZeroUsize::new(1).unwrap()).get();
        let mut threads = vec![];

        println!("Starting with {} threads.", thread_count);
        for _ in 0..thread_count {
            threads.push(Self::spawn_thread(&handler, &recv));
        }

        loop { 
            let stream = match self.listener.accept() {
                Err(e) => {
                    eprintln!("Failed to establish a connection {}", e);
                    continue;
                },
                Ok((stream, _)) => stream,
            };

            send.send(stream).expect("Failed to send TcpStream to threads");
        }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    fn spawn_thread(handler: &Arc<impl RequestHandler + Send + Sync + 'static>, receiver: &Receiver<TcpStream>) -> JoinHandle<()> {
        let handler = handler.clone();
        let recv = receiver.clone();
        thread::spawn(move || {
            loop {
                let msg = recv.recv();
                if let Err(e) = &msg {
                    eprintln!("Failed to recieve message from channel: {}", e);
                    continue;
                }
                let mut bytes = [0; (2 as usize).pow(10)];
                let stream = Rc::new(RefCell::new(msg.unwrap()));
                let mut response = Response::new(stream.clone());
                
                let result = match Self::decode(stream.clone(), &mut bytes) {
                    Ok(mut req) => handler.handle(&mut req, &mut response),
                    Err(err) => handler.handle_bad(&mut response, &err.to_string())
                };

                if let Err(e) = result {
                    eprintln!("Something went wrong sending response:{}\n{:?}", e, response);
                }
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

