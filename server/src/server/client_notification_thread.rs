use std::io::Write;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use crate::Server;
use crate::server::core::notification::Notification;

/// This thread handle packet sending to client when packet are not a response from a direct client request.

impl Server {
    pub fn build(server_ref: Arc<Server>, response_receiver: Receiver<Notification>) -> JoinHandle<()> {
        thread::Builder::new().name("client-notification-thread".to_string()).spawn(move || {
            for response in response_receiver.iter() {
                match response {
                    Notification::Char(char_notification) => {
                        let tcp_stream = server_ref.get_map_socket_for_account_id(char_notification.account_id()).expect("Expect to found a socket for account");
                        let data = char_notification.serialized_packet();
                        let mut tcp_stream_guard = tcp_stream.write().unwrap();
                        debug!("Respond with: {:02X?}", data);
                        tcp_stream_guard.write_all(data).unwrap();
                        tcp_stream_guard.flush().unwrap();
                    }
                    Notification::Area(_) => {}
                }
            }
        }).expect("Failed to create client-notification")
    }
}