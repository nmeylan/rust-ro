use std::io::Read;
use std::net::{Shutdown, TcpListener};
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{SyncSender};
use std::thread;
use std::thread::JoinHandle;
use tokio::runtime::Runtime;
use packets::packets_parser::parse;
use crate::Server;
use crate::server::core::request::Request;
use crate::server::core::response::Response;
use crate::server::server::PACKETVER;

pub fn build(server_ref: Arc<Server>, port: u16, response_sender: SyncSender<Response>) -> JoinHandle<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    info!("Server listen on 0.0.0.0:{}", port);
    let server_shared_ref = server_ref;
    thread::Builder::new().name("server-incoming-connection".to_string()).spawn(move || {
        for tcp_stream in listener.incoming() {
            // Receive new connection, starting new thread
            let server_shared_ref = server_shared_ref.clone();
            debug!("Received new connection");
            let response_sender_clone= response_sender.clone();
            thread::Builder::new().name("request-thread".to_string()).spawn(move || {
                PACKETVER.with(|ver| *ver.borrow_mut() = server_shared_ref.packetver());
                let runtime = Runtime::new().unwrap();
                let mut tcp_stream = tcp_stream.unwrap();
                let tcp_stream_arc = Arc::new(RwLock::new(tcp_stream.try_clone().unwrap())); // todo remove this clone
                let mut buffer = [0; 2048];
                loop {
                    match tcp_stream.read(&mut buffer) {
                        Ok(bytes_read) => {
                            if bytes_read == 0 {
                                tcp_stream.shutdown(Shutdown::Both).expect("Unable to shutdown incoming socket. Shutdown was done because remote socket seems cloded.");
                                break;
                            }
                            let mut packet = parse(&mut buffer[..bytes_read], server_shared_ref.packetver());
                            let context = Request::new(&runtime, None, server_shared_ref.packetver(), tcp_stream_arc.clone(), packet.as_ref(), response_sender_clone.clone());
                            server_shared_ref.handle(server_shared_ref.clone(), context);
                        }
                        Err(err) => error!("{}", err)
                    }
                }
            }).expect("Failed to create sever-handle-request thread");
        }
    }).expect("Failed to create sever-incoming-connection thread")
}