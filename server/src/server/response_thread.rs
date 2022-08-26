use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use crate::server::core::response::Response;
use std::io::Write;

pub fn build(response_receiver: Receiver<Response>) -> JoinHandle<()> {
    thread::Builder::new().name("server-response-thread".to_string()).spawn(move || {
        for response in response_receiver.iter() {
            let tcp_stream = &response.socket();
            let data = response.serialized_packet();
            let mut tcp_stream_guard = tcp_stream.write().unwrap();
            debug!("Respond with: {:02X?}", data);
            tcp_stream_guard.write_all(data).unwrap();
            tcp_stream_guard.flush().unwrap();
        }
    }).expect("Failed to create server-response-thread")
}