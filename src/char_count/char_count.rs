use std::{sync::{atomic::AtomicU32, Arc, mpsc, Mutex}, net::TcpStream, thread::{JoinHandle, self}};

use tungstenite::{WebSocket, stream::MaybeTlsStream};

use crate::{server::server::WebSocketServer, worker::worker::WebSocketClient};

use super::fake_text::generate_fake_text;

pub fn char_count(text: &str) -> usize
{
    text.len()
}

fn add_chars_to_count(text: String) -> impl Fn(WebSocket<MaybeTlsStream<TcpStream>>) + Clone
{
    let char_count = char_count(&text);
    return move |mut socket: WebSocket<MaybeTlsStream<TcpStream>>| {
        socket
            .write_message(tungstenite::Message::text(char_count.to_string()))
            .expect("Failed to send message");
    };
}

fn accumulate_chars(counter: Arc<AtomicU32>) -> impl Fn(WebSocket<TcpStream>) + Clone
{
    let counter_clone = counter.clone();
    move |mut ws_stream: WebSocket<std::net::TcpStream>| {
        while let Ok(msg) = ws_stream.read_message() {
            if msg.is_text() {
                let msg_text = msg.to_text().unwrap_or("");
                println!("Received text: {}", msg_text);
                if msg_text.is_empty() {
                    return;
                }
                let count = msg_text.parse::<u32>().unwrap_or(0);
                counter_clone.fetch_add(count, std::sync::atomic::Ordering::SeqCst);
                println!("Character count: {}", counter_clone.load(std::sync::atomic::Ordering::SeqCst));
            }
        }
    }
}

pub fn distributed_character_count(n_clients: usize)
{
    // Create a channel for communication
    let (sender, receiver) = mpsc::channel();
    let sender = Arc::new(Mutex::new(sender));

    let character_count = Arc::new(AtomicU32::new(0));

    // We won't wait for the server to finish.
    thread::spawn(move || {
        // Start WebSocket server
        let server = WebSocketServer::new("127.0.0.1:8080", accumulate_chars(character_count.clone()));
        server.run(&sender);
    });

    let n_threads = n_clients;
    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    receiver.recv().unwrap();
    for n_thread in 0..n_threads
    {
        let client_thread = thread::spawn(move || {
            // Start WebSocket client
            let mut client = WebSocketClient::new("ws://127.0.0.1:8080", add_chars_to_count(generate_fake_text(n_thread + 1)));
            client.run();
        });

        threads.push(client_thread);
    }

    for handle in threads
    {
        handle.join().expect("Client thread died");
    }
}