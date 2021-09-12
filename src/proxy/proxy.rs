use std::net::{SocketAddr, TcpListener, TcpStream, Shutdown};
use std::thread::{JoinHandle, spawn};
use std::io::{Read, Write};
use std::thread;
use std::fmt::{Display, Formatter, Debug};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::packets::packets::{Packet};
use crate::packets::packets_parser::parse;
use crate::server::core::{Server, FeatureState};

#[derive(Clone)]
pub struct Proxy<T: PacketHandler + Clone + Send> {
    pub name: String,
    pub server: Arc<Server>,
    pub local_port: u16,
    pub target: SocketAddr,
    pub specific_proxy: T,
}

pub trait PacketHandler {
    fn handle_packet(&self, tcp_stream: Arc<Mutex<TcpStream>>, packet: &mut dyn Packet) -> Result<String, String>;
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

impl<T: 'static + PacketHandler + Clone + Send + Sync> Proxy<T> {
    pub fn proxy(&self) -> JoinHandle<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.local_port)).unwrap();
        let immutable_self_ref = Arc::new(self.clone()); // make An Arc of self to be able to share it with other threads
        let server_ref = immutable_self_ref.clone(); // cloning the ref to use in thread below
        spawn(move || {
            println!("Start proxy for {} proxy, {}:{}", server_ref.name, server_ref.local_port, server_ref.target.port());
            for tcp_stream in listener.incoming() {
                let server_ref = immutable_self_ref.clone(); // cloning the ref to use in thread below
                // Receive new connection, starting new thread
                spawn(move || {
                    // Use Arc to be able to share reference across thread.
                    // Arc are immutable, use a mutex to be able to mutate arc value.
                    if let Err(error) = server_ref.proxy_connection(tcp_stream.unwrap()) {
                        println!("{}", error);
                    }
                });
            }
        })
    }

    fn proxy_connection(&self, incoming_stream: TcpStream) -> Result<(), String> {
        let mut forward_thread_incoming = incoming_stream.try_clone().unwrap();
        println!("Client connected from: {:#?} to {:#?}", incoming_stream.peer_addr().unwrap(), incoming_stream.local_addr().unwrap());

        let mut forward_thread_outgoing = TcpStream::connect(self.target)
            .map_err(|error| format!("Could not establish connection to {}: {}", self.target, error))?;

        let mut backward_thread_incoming_clone = incoming_stream.try_clone().map_err(|e| e.to_string())?;
        let mut backward_thread_outgoing_clone = forward_thread_outgoing.try_clone().map_err(|e| e.to_string())?;
        let server_copy_forward_thread = self.clone();
        let server_copy_backward_thread = self.clone();
        // Pipe for- and backward asynchronously
        let forward = thread::Builder::new().name(format!("{}-{}", self.name, "forward"))
            .spawn(move || server_copy_forward_thread.pipe(&mut forward_thread_incoming, &mut forward_thread_outgoing, ProxyDirection::Forward)).unwrap();
        let backward = thread::Builder::new().name(format!("{}-{}", self.name, "backward"))
            .spawn(move || server_copy_backward_thread.pipe(&mut backward_thread_outgoing_clone, &mut backward_thread_incoming_clone, ProxyDirection::Backward)).unwrap();
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
                    let tcp_stream_ref = Arc::new(Mutex::new(incoming.try_clone().unwrap()));
                    let mut packet = parse(&mut buffer[..bytes_read]);
                    let feature_state = self.server.dispatch(packet.as_mut());
                    match feature_state {
                        FeatureState::Unimplemented => {
                            print!("{} {} ", self.name, if direction == ProxyDirection::Backward { "<" } else { ">" });
                            self.specific_proxy.handle_packet(tcp_stream_ref, packet.as_mut());
                            if outgoing.write(packet.raw()).is_ok() {
                                outgoing.flush();
                            }
                        }
                        FeatureState::Implemented(response_packet) => {
                            packet = response_packet;
                            if incoming.write(packet.raw()).is_ok() {
                                incoming.flush();
                            }
                        }
                    }
                }
                Err(error) => return Err(format!("Could not read data: {}", error))
            }
        }
        Ok(())
    }
}