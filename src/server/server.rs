use std::net::{SocketAddr, TcpListener, TcpStream, Shutdown};
use std::thread::{JoinHandle, spawn};
use std::io::{Read, Write, Bytes, Cursor};
use std::thread;
use byteorder::{ReadBytesExt, LittleEndian, WriteBytesExt};
use std::time::{Instant, SystemTime};

#[derive(Clone)]
pub struct Server<T : PacketHandler + Clone + Send> {
    pub name: String,
    pub local_port: u16,
    pub target: SocketAddr,
    pub packet_handler: T
}

pub trait PacketHandler {
    fn handle_packet(&self, packet: &mut [u8]) -> Result<String, String>;
}

#[derive(Clone)]
pub struct LoginServer;
#[derive(Clone)]
pub struct CharServer;
#[derive(Clone)]
pub struct MapServer;

impl PacketHandler for LoginServer {

    fn handle_packet(&self, packet: &mut [u8]) -> Result<String, String> {
        println!("Login");
        Result::Ok("res".to_string())
    }
}
impl PacketHandler for CharServer {
    fn handle_packet(&self, packet: &mut [u8]) -> Result<String, String> {
        println!("Char");
        Result::Ok("res".to_string())
    }
}
impl PacketHandler for MapServer {
    fn handle_packet(&self, packet: &mut [u8]) -> Result<String, String> {
        println!("Map");
        Result::Ok("res".to_string())
    }
}

impl <T : 'static + PacketHandler + Clone + Send> Server<T> {
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

        // Pipe for- and backward asynchronously
        let server = self.clone();
        let server1 = self.clone();
        let forward = thread::Builder::new().name("forward".into()).spawn(move || server.pipe(&mut incomingStream, &mut outgoing)).unwrap();
        let backward = thread::Builder::new().name("backward".into()).spawn(move || server1.pipe(&mut outgoing_clone, &mut incoming_clone)).unwrap();
        println!("Proxying data...");
        forward.join().map_err(|error| format!("Forward failed: {:?}", error))?;
        backward.join().map_err(|error| format!("Backward failed: {:?}", error))?;

        println!("Socket closed");

        Ok(())
    }

    fn pipe(&self, incoming: &mut TcpStream, outgoing: &mut TcpStream) -> Result<(), String> {
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
                    let from: SocketAddr;
                    let to: SocketAddr;
                    let mut direction = "";
                    match thread::current().name().unwrap() {
                        "forward" => {
                            direction = " > ";
                            from = incoming.local_addr().unwrap();
                            to = outgoing.peer_addr().unwrap();
                        }
                        "backward" => {
                            direction = " < ";
                            from = incoming.peer_addr().unwrap();
                            to = outgoing.local_addr().unwrap();
                        }
                        _ => panic!()
                    }
                    if packet[0] == 0xc4 && packet[1] == 0x0a {
                        // PACKET_AC_ACCEPT_LOGIN
                        // char server IP is in bytes 64..68
                        // char server port is in bytes 68..70
                        println!("{:x?}", &packet[64..70]);
                        let mut wtr = Vec::new();
                        wtr.write_u8(0xeb).unwrap();
                        packet[68] = wtr[0];
                    }
                    if packet[0] == 0xc5 && packet[1] == 0x0a {
                        // char_send_map_info
                        // map server IP is in bytes 22..26
                        // map server port is in bytes 26..28
                        println!("{:x?}", &packet[22..28]);
                        let mut wtr = Vec::new();
                        wtr.write_u8(0xec).unwrap();
                        packet[26] = wtr[0];
                    }
                    self.packet_handler.handle_packet(packet);
                    println!("{}{}({}) {:x?}", self.name, direction, bytes_read, packet);
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
