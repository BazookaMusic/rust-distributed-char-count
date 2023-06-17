use std::net::TcpStream;

use tungstenite::{connect, WebSocket, stream::MaybeTlsStream};

pub struct WebSocketClient<T>
where
    T: FnMut(WebSocket<MaybeTlsStream<TcpStream>>) + Send + 'static + Clone,
{
    server_address: String,
    operation: T
}

impl<T> WebSocketClient<T> 
where
    T: FnMut(WebSocket<MaybeTlsStream<TcpStream>>) + Send + 'static + Clone,
{
    pub fn new(server_address: &str, op: T) -> Self {
        WebSocketClient {
            server_address: server_address.to_owned(),
            operation: op
        }
    }

    pub fn run(&mut self) {
        let (socket, _) = connect(&self.server_address)
            .expect("Failed to connect to WebSocket server");

        (self.operation)(socket);
    }
}