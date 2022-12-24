use std::sync::{Arc, Mutex};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use packets::packets::{Packet};
use std::fmt::{Display, Formatter};
use std::thread::{JoinHandle, spawn};
use std::thread;
use tokio::runtime::Runtime;
use packets::packets_parser::parse;
use std::io::{Read, Write};

pub mod map;
pub mod char;

#[derive(Clone)]
pub struct Proxy<T: PacketHandler + Clone + Send> {
    pub name: String,
    pub local_port: u16,
    pub target: SocketAddr,
    pub specific_proxy: T,
}

pub trait PacketHandler {
    fn handle_packet(&self, tcp_stream: Arc<Mutex<TcpStream>>, packet: &mut dyn Packet);
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
    pub fn proxy(&self, packetver: u32) -> JoinHandle<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.local_port)).unwrap();
        let immutable_self_ref = Arc::new(self.clone()); // make An Arc of self to be able to share it with other threads
        let server_ref = immutable_self_ref.clone(); // cloning the ref to use in thread below
        spawn(move || {
            info!("Start proxy for {} proxy, {}:{}", server_ref.name, server_ref.local_port, server_ref.target.port());
            for tcp_stream in listener.incoming() {
                let server_ref = immutable_self_ref.clone(); // cloning the ref to use in thread below
                // Receive new connection, starting new thread
                spawn(move || {
                    // Use Arc to be able to share reference across thread.
                    // Arc are immutable, use a mutex to be able to mutate arc value.
                    server_ref.proxy_connection(tcp_stream.unwrap(), packetver);
                });
            }
        })
    }

    fn proxy_connection(&self, incoming_stream: TcpStream, packetver: u32) {
        let mut forward_thread_incoming = incoming_stream.try_clone().unwrap();
        debug!("Client connected from: {:#?} to {:#?}", incoming_stream.peer_addr().unwrap(), incoming_stream.local_addr().unwrap());

        let mut forward_thread_outgoing = TcpStream::connect(self.target)
            .unwrap_or_else(|_| panic!("Could not establish connection to {}", self.target));

        let mut backward_thread_incoming_clone = incoming_stream.try_clone().expect("Unable to clone incoming tcp stream for proxy");
        let mut backward_thread_outgoing_clone = forward_thread_outgoing.try_clone().expect("Unable to clone outgoing tcp stream for proxy");
        let mut server_copy_forward_thread = self.clone();
        let mut server_copy_backward_thread = self.clone();
        // Pipe for- and backward asynchronously
        let forward = thread::Builder::new().name(format!("{}-{}", self.name, "forward"))
            .spawn(move || {
                let rt = Runtime::new().unwrap();
                server_copy_forward_thread.pipe(&mut forward_thread_incoming, &mut forward_thread_outgoing, ProxyDirection::Forward, &rt, packetver)
            }).unwrap();
        let backward = thread::Builder::new().name(format!("{}-{}", self.name, "backward"))
            .spawn(move || {
                let rt = Runtime::new().unwrap();
                server_copy_backward_thread.pipe(&mut backward_thread_outgoing_clone, &mut backward_thread_incoming_clone, ProxyDirection::Backward, &rt, packetver)
            }).unwrap();
        let _ = forward.join().expect("Forward failed");
        let _ = backward.join().expect("Backward failed");

        debug!("Socket closed");

    }

    fn pipe(&mut self, incoming: &mut TcpStream, outgoing: &mut TcpStream, direction: ProxyDirection, _runtime: &Runtime, packetver: u32) -> Result<(), String> {
        let mut buffer = [0; 2048];
        loop {
            // println!("loop direction {} incoming peer {} incoming local {} outgoing local {} outgoing peer {} ", direction,
            //          incoming.peer_addr().unwrap(), incoming.local_addr().unwrap(),
            //          outgoing.local_addr().unwrap(), outgoing.peer_addr().unwrap(),
            // );
            match incoming.read(&mut buffer) {
                Ok(bytes_read) => {
                    // no more data
                    if bytes_read == 0 {
                        debug!("shutdown {} direction {}", outgoing.local_addr().unwrap(), direction);
                        outgoing.shutdown(Shutdown::Both).expect("Failed to shutdown incoming socket for proxy");
                        incoming.shutdown(Shutdown::Both).expect("Failed to shutdown incoming socket for proxy");
                        break;
                    }
                    let tcp_stream_ref = Arc::new(Mutex::new(incoming.try_clone().unwrap()));
                    let mut packet = parse(&buffer[..bytes_read], packetver);
                    if packet.raw().len() < bytes_read {
                        let mut offset = 0;
                        while offset < bytes_read {
                            packet = parse(&buffer[offset..bytes_read], packetver);
                            offset += packet.raw().len();
                            self.proxy_request(outgoing, &direction, tcp_stream_ref.clone(), packet);
                        }
                    } else {
                        self.proxy_request(outgoing, &direction, tcp_stream_ref, packet);
                    }
                }
                Err(error) => return Err(format!("Could not read data: {}", error))
            }
        }
        Ok(())
    }

    fn proxy_request(&self, outgoing: &mut TcpStream, direction: &ProxyDirection, tcp_stream_ref: Arc<Mutex<TcpStream>>, mut packet: Box<dyn Packet>) {
        if packet.id() != "0x6003" && packet.id() != "0x7f00" && packet.id() != "0x7e00" { // PACKET_CZ_REQUEST_TIME2
            info!("{} {} {} ", self.name, if *direction == ProxyDirection::Backward { "<" } else { ">" }, outgoing.peer_addr().unwrap());
            self.specific_proxy.handle_packet(tcp_stream_ref, packet.as_mut());
            packet.display();
            packet.pretty_debug();
            info!("{:02X?}", packet.raw());

        }
        if outgoing.write(packet.raw()).is_ok() {
            outgoing.flush().expect("Failed to flush packet for outgoing socket to proxied server");
        }
    }
}
