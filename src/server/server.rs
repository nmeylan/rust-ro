use std::net::{SocketAddr, TcpListener, TcpStream, Shutdown};
use std::thread::{JoinHandle, spawn};
use std::io::{Read, Write, Bytes, Cursor};
use std::thread;
use byteorder::{ReadBytesExt, LittleEndian, WriteBytesExt};
use std::time::{Instant, SystemTime};
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Server<T: PacketHandler + Clone + Send> {
    pub name: String,
    pub local_port: u16,
    pub target: SocketAddr,
    pub packet_handler: T,
}

pub trait PacketHandler {
    fn handle_packet(&self, packet: &mut [u8]) -> Result<String, String>;
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

impl<T: 'static + PacketHandler + Clone + Send> Server<T> {
    pub fn proxy(&self) -> JoinHandle<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.local_port)).unwrap();
        let server = self.clone();
        spawn(move || {
            println!("Start proxy for {} server, {}:{}", server.name, server.local_port, server.target.port());
            for socket in listener.incoming() {
                // One thread per connection
                let server = server.clone();
                spawn(move || {
                    if let Err(error) = server.proxy_connection(socket.unwrap()) {
                        println!("{}", error);
                    }
                });
            }
        })
    }

    fn proxy_connection(&self, mut incomingStream: TcpStream) -> Result<(), String> {
        println!("Client connected from: {:#?} to {:#?}", incomingStream.peer_addr().unwrap(), incomingStream.local_addr().unwrap());

        let mut outgoing = TcpStream::connect(self.target)
            .map_err(|error| format!("Could not establish connection to {}: {}", self.target, error))?;

        let mut incoming_clone = incomingStream.try_clone().map_err(|e| e.to_string())?;
        let mut outgoing_clone = outgoing.try_clone().map_err(|e| e.to_string())?;

        let server_copy_forward_thread = self.clone();
        let server_copy_backward_thread = self.clone();
        // Pipe for- and backward asynchronously
        let forward = thread::Builder::new().name(format!("{}-{}", self.name, "forward"))
            .spawn(move || server_copy_forward_thread.pipe(&mut incomingStream, &mut outgoing, ProxyDirection::Forward)).unwrap();
        let backward = thread::Builder::new().name(format!("{}-{}", self.name, "backward"))
            .spawn(move || server_copy_backward_thread.pipe(&mut outgoing_clone, &mut incoming_clone, ProxyDirection::Backward)).unwrap();
        println!("Proxying data...");
        forward.join().map_err(|error| format!("Forward failed: {:?}", error))?;
        backward.join().map_err(|error| format!("Backward failed: {:?}", error))?;

        println!("Socket closed");

        Ok(())
    }

    fn pipe(&self, incoming: &mut TcpStream, outgoing: &mut TcpStream, direction: ProxyDirection) -> Result<(), String> {
        let mut buffer = [0; 1024];
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
                    println!("{} {} ({}) {:x?}", self.name, if direction == ProxyDirection::Backward { "<" } else { ">" }, bytes_read, packet);
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
