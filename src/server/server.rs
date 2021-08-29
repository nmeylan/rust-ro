use std::net::{SocketAddr, TcpListener, TcpStream, Shutdown};
use std::thread::{JoinHandle, spawn};
use std::io::{Read, Write, Bytes, Cursor};
use std::thread;
use byteorder::{ReadBytesExt, LittleEndian, WriteBytesExt};
use std::time::{Instant, SystemTime};
use std::fmt::{Display, Formatter};
use crate::server::packet_db::PACKETS_DB;
use crate::server::packet_parser::packet_name;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::borrow::{BorrowMut, Borrow};
use std::ops::{DerefMut, Deref};

#[derive(Clone)]
pub struct Server<T: PacketHandler + Clone + Send> {
    pub name: String,
    pub local_port: u16,
    pub target: SocketAddr,
    pub packet_handler: T,
}

pub struct ServerContext {

}

pub trait PacketHandler {
    fn handle_packet(&self, packet: &mut [u8]) -> Result<String, String>;
    fn handle_connection(&mut self, tcp_stream: Arc<Mutex<TcpStream>>) {
        println!("New connection");
    }
}

#[derive(Debug, PartialEq)]
enum ProxyDirection {
    Forward,
    Backward,
}

impl Display for ProxyDirection {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T: 'static + PacketHandler + Clone + Send + Sync> Server<T> {
    pub fn proxy(&self) -> JoinHandle<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.local_port)).unwrap();
        let mutable_self_ref = Arc::new(Mutex::new(self.clone())); // make An Arc of self to be able to share it with other threads
        let immutable_self_ref = Arc::new(self.clone()); // make An Arc of self to be able to share it with other threads
        let server_ref = immutable_self_ref.clone(); // cloning the ref to use in thread below
        spawn(move || {
            println!("Start proxy for {} server, {}:{}", server_ref.name, server_ref.local_port, server_ref.target.port());
            for tcp_stream in listener.incoming() {
                let mut server_mutex = mutable_self_ref.clone();  // make An Arc of self to be able to share it with other threads
                let server_ref = immutable_self_ref.clone(); // cloning the ref to use in thread below
                // Receive new connection, starting new thread
                spawn(move || {
                    let mut self_ref_guard = server_mutex.lock().unwrap();
                    // Use Arc to be able to share reference across thread.
                    // Arc are immutable, use a mutex to be able to mutate arc value.
                    let tcp_stream_ref = Arc::new(Mutex::new(tcp_stream.unwrap()));
                    self_ref_guard.packet_handler.handle_connection(tcp_stream_ref.clone());
                    drop(self_ref_guard);
                    if let Err(error) = server_ref.proxy_connection(tcp_stream_ref.clone()) {
                        println!("{}", error);
                    }
                });
            }
        })
    }

    fn proxy_connection(&self, mut incoming_stream: Arc<Mutex<TcpStream>>) -> Result<(), String> {
        let mut stream_ref = incoming_stream.lock().unwrap();
        let mut stream_ref_clone = stream_ref.try_clone().unwrap();
        println!("Client connected from: {:#?} to {:#?}", stream_ref.peer_addr().unwrap(), stream_ref.local_addr().unwrap());

        let mut outgoing = TcpStream::connect(self.target)
            .map_err(|error| format!("Could not establish connection to {}: {}", self.target, error))?;

        let mut incoming_clone = stream_ref.try_clone().map_err(|e| e.to_string())?;
        let mut outgoing_clone = outgoing.try_clone().map_err(|e| e.to_string())?;

        let server_copy_forward_thread = self.clone();
        let server_copy_backward_thread = self.clone();
        // Pipe for- and backward asynchronously
        let forward = thread::Builder::new().name(format!("{}-{}", self.name, "forward"))
            .spawn(move || server_copy_forward_thread.pipe(&mut stream_ref_clone, &mut outgoing, ProxyDirection::Forward)).unwrap();
        let backward = thread::Builder::new().name(format!("{}-{}", self.name, "backward"))
            .spawn(move || server_copy_backward_thread.pipe(&mut outgoing_clone, &mut incoming_clone, ProxyDirection::Backward)).unwrap();
        println!("Proxying data...");
        forward.join().map_err(|error| format!("Forward failed: {:?}", error))?;
        backward.join().map_err(|error| format!("Backward failed: {:?}", error))?;

        println!("Socket closed");

        Ok(())
    }

    fn pipe(&self, incoming: &mut TcpStream, outgoing: &mut TcpStream, direction: ProxyDirection) -> Result<(), String> {
        let mut buffer = [0; 2048];
        loop {
            match incoming.read(&mut buffer) {
                Ok(bytes_read) => {
                    // no more data
                    if bytes_read == 0 {
                        outgoing.shutdown(Shutdown::Both);
                        break;
                    }
                    let mut packet = &mut buffer[..bytes_read];

                    self.packet_handler.handle_packet(packet);
                    println!("{} {} {} ({}) {:02X?} {}",
                             self.name,
                             if direction == ProxyDirection::Backward { "<" } else { ">" },
                             packet_name(packet),
                             bytes_read,
                             packet, if  direction == ProxyDirection::Backward { outgoing.peer_addr().unwrap() } else { incoming.peer_addr().unwrap() });
                    if outgoing.write(packet).is_ok() {
                        outgoing.flush();
                    }
                }
                Err(error) => return Err(format!("Could not read data: {}", error))
            }
        }
        Ok(())
    }

}
