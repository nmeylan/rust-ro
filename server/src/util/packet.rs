use crate::server::service::global_config_service::GlobalConfigService;
use crate::util::tick::get_tick;
use packets::packets::Packet;
use packets::packets_parser::parse;
use std::fmt::{Display, Formatter};
use std::net::SocketAddr;
use std::panic;

pub fn chain_packets(packets: Vec<&dyn Packet>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for packet in packets {
        res = res.iter().cloned().chain(packet.raw().iter().cloned()).collect();
    }
    res
}

#[allow(dead_code)]
pub fn chain_packets_raws(packets_raw: Vec<&Vec<u8>>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for raw in packets_raw {
        res = res.iter().cloned().chain(raw.iter().cloned()).collect();
    }
    res
}

pub fn chain_packets_raws_by_value(packets_raw: Vec<Vec<u8>>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for raw in packets_raw {
        res = res.iter().cloned().chain(raw).collect();
    }
    res
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PacketDirection {
    Forward,
    Backward,
}

impl Display for PacketDirection {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub fn debug_packets_from_vec(outgoing: Option<&SocketAddr>, direction: PacketDirection, packetver: u32, bytes: &Vec<u8>, name: &Option<String>) {
    let mut buffer = [0_u8; 20048];
    if bytes.len() < 2 {
        return;
    }
    if bytes.len() > buffer.len() {
        panic!("Bulk of packet is greater {} than buffer {}", bytes.len(), buffer.len());
    }
    for (i, byte) in bytes.iter().enumerate() {
        if i >= buffer.len() {
            break;
        }
        buffer[i] = *byte;
    }
    if bytes.len() >= buffer.len() {
        panic!("Can't debug packet {:02X?}\n from bulk packets {:02X?}", buffer, bytes.as_slice())
    }
    debug_packets(outgoing, direction, packetver, &buffer, bytes.len(), name);
}
pub fn debug_packets(outgoing: Option<&SocketAddr>, direction: PacketDirection, packetver: u32, buffer: &[u8], bytes_read: usize, name: &Option<String>) {
    let packet = parse(&buffer[..bytes_read], packetver);
    if packet.raw().len() < bytes_read {
        let mut offset = 0;
        while offset < bytes_read {
            let result = panic::catch_unwind(|| {
                parse(&buffer[offset..bytes_read], packetver)
            });
            if let Ok(packet) = result {
                offset += packet.raw().len();
                print_packet(name, outgoing, direction, &packet);
            } else {
                break;
            }
        }
    } else {
        print_packet(name, outgoing, direction, &packet);
    }
}


pub fn print_packet(name: &Option<String>, outgoing: Option<&SocketAddr>, direction: PacketDirection, packet: &Box<dyn Packet>) {
    if packet.id(GlobalConfigService::instance().packetver()) != "0x6003"
        && packet.id(GlobalConfigService::instance().packetver()) != "0x7f00"
        && packet.id(GlobalConfigService::instance().packetver()) != "0x0887"
        && packet.id(GlobalConfigService::instance().packetver()) != "0x7e00" { // PACKET_CZ_REQUEST_TIME2
        println!("\n----------------------------Start Packet----------------------------");
        if let Some(outgoing) = outgoing {
            info!("{} {} {}", name.as_ref().cloned().unwrap_or("server".to_string()), if direction == PacketDirection::Backward { "<" } else { ">" }, outgoing);
        } else {
            info!("{} {} Client", name.as_ref().cloned().unwrap_or("".to_string()), if direction == PacketDirection::Backward { "<" } else { ">" });
        }
        packet.display();
        packet.pretty_debug();
        info!("{:02X?}", packet.raw());
        println!("----------------------------End Packet----------------------------\n");
    }
}

/**
 Packets are buffered by session and flushed to the TCP Socket after a short delay.
 It allows to "send" packets from anywhere from the code, in the end packets will be send by batch and not one by one.
*/
pub struct PacketsBuffer {
    session_id: u32,
    max_capacity: usize,
    created_at: u128,
    data: Vec<u8>
}

impl PacketsBuffer {
    pub fn new(session_id: u32, max_capacity: usize) -> Self {
        Self {
            session_id,
            max_capacity,
            created_at: get_tick(),
            data: Vec::with_capacity(2048),
        }
    }

    pub fn push(&mut self, new_data: &[u8]) {
        self.data.extend_from_slice(new_data);
    }
    pub fn should_flush(&self) -> bool {
        get_tick() - self.created_at > 20
    }

    pub fn can_contain(&self, len: usize) -> bool {
        self.data.len() + len <= self.max_capacity
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
    pub fn session_id(&self) -> u32 {
        self.session_id
    }
}