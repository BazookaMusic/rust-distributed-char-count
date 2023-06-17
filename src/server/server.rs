use std::net::TcpListener;
use std::sync::{mpsc, Arc, Mutex};

use tungstenite::accept;
use tungstenite::protocol::WebSocket;
use rayon::ThreadPool;
use rayon::ThreadPoolBuilder;
use num_cpus;

pub struct WebSocketServer<T>
where 
T: FnMut(WebSocket<std::net::TcpStream>) + Send + 'static,
{
    address: String,
    thread_pool: ThreadPool,
    handle_connection: T
}


impl<T> WebSocketServer<T>
where
    T: FnMut(WebSocket<std::net::TcpStream>) + Send + 'static + Clone,
{
    pub fn new(address: &str, handle_connection: T) -> Self {
        let num_thread = num_cpus::get();
        let thread_pool = ThreadPoolBuilder::new()
        .num_threads(num_thread)
        .build()
        .unwrap();

        WebSocketServer {
            address: address.to_owned(),
            thread_pool,
            handle_connection
        }
    }

    pub fn run(&self, server_ready: &Arc<Mutex<mpsc::Sender<()>>>) {
        let server = TcpListener::bind(&self.address).expect("Failed to bind address");
        println!("WebSocket server started on {}", self.address);

        server_ready.lock().unwrap().send(()).unwrap();

        for stream in server.incoming() {
            match stream {
                Ok(stream) => {
                    let ws_stream = accept(stream).expect("Failed to accept connection");
                    let mut handle_connection_ref = self.handle_connection.clone();
                    self.thread_pool.spawn(move || {
                        handle_connection_ref(ws_stream);
                    });
                }
                Err(err) => {
                    println!("Error accepting connection: {:?}", err);
                }
            }
        }
    }
}