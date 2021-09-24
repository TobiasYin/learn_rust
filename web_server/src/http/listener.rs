use std::net::TcpListener;
use std::thread;
use crate::http::handler::Handler;
use crate::http::core::handle_connection;
use std::process::exit;

pub fn listen<T: 'static + Handler>(addr: &str, handler: T) -> !{
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let new_handler = handler.clone();
        thread::spawn(move ||{
            handle_connection(stream, new_handler);
        });
    }
    exit(0)
}